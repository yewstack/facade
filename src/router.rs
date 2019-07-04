use failure::Error;
use futures3::channel::mpsc;
use futures3::executor::block_on;
use futures3::stream::select;
use futures3::{SinkExt, StreamExt};
use protocol::{Delta, Id, Layout, Reaction, Value};
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
    SetLayout(Layout),
    SetValue(Delta),
}

pub type Response = Reaction;

pub async fn main(mut receiver: Receiver) -> Result<(), Error> {
    let mut subscribers = Vec::new();
    let mut board = HashMap::<Id, Value>::new();
    let mut layout = Layout::Welcome;
    while let Some(request) = receiver.rx.next().await {
        let mut drain_all = false;
        match request {
            Request::Subscribe(mut sender) => {
                // Send layout to a new subscriber
                let response = Reaction::Layout(layout.clone());
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
            Request::SetLayout(new_layout) => {
                // Send new_layout to every subscriber
                layout = new_layout;
                let response = Reaction::Layout(layout.clone());
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
