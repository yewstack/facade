use failure::Error;
use futures3::channel::mpsc;
use futures3::{SinkExt, StreamExt};
use protocol::{Delta, Id, Reaction, Scene, Value};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<Request>,
}

impl Sender {
    pub fn wrap(tx: mpsc::Sender<Request>) -> Self {
        Self { tx }
    }

    pub async fn register(&mut self) -> Result<mpsc::Receiver<Response>, Error> {
        let (tx, rx) = mpsc::channel(8);
        let request = Request::Subscribe(tx);
        self.tx.send(request).await?;
        Ok(rx)
    }
}

pub struct Receiver {
    rx: mpsc::Receiver<Request>,
}

impl Receiver {
    pub fn wrap(rx: mpsc::Receiver<Request>) -> Self {
        Self { rx }
    }
}

pub enum Request {
    Subscribe(mpsc::Sender<Response>),
    SetScene(Scene),
    SetValue(Delta),
}

pub type Response = Reaction;

pub async fn main(mut receiver: Receiver) -> Result<(), Error> {
    let mut subscribers = Vec::new();
    let mut board = HashMap::<Id, Value>::new();
    let mut scene = Scene::Spinner;
    while let Some(request) = receiver.rx.next().await {
        let mut drain_all = false;
        match request {
            Request::Subscribe(mut sender) => {
                // Send scene to a new subscriber
                let response = Reaction::Scene(scene.clone());
                drain_all |= sender.send(response).await.is_err();
                let snapshot = board
                    .iter()
                    .map(|(id, value)| Delta::from((id.clone(), value.clone())));
                for delta in snapshot {
                    let response = Reaction::Delta(delta);
                    drain_all |= sender.send(response).await.is_err();
                }
                subscribers.push(sender);
            }
            Request::SetScene(new_scene) => {
                // Send new_scene to every subscriber
                scene = new_scene;
                let response = Reaction::Scene(scene.clone());
                for sender in &mut subscribers {
                    drain_all |= sender.send(response.clone()).await.is_err();
                }
            }
            Request::SetValue(delta) => {
                board.insert(delta.id.clone(), delta.value.clone());
                let response = Reaction::Delta(delta);
                for sender in &mut subscribers {
                    drain_all |= sender.send(response.clone()).await.is_err();
                }
            }
        }
        if drain_all {
            subscribers.retain(|sender| !sender.is_closed());
        }
    }
    Ok(())
}
