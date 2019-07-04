use failure::Error;
use futures3::channel::mpsc;
use futures3::stream::select;
use futures3::{SinkExt, StreamExt};
use protocol::{Id, Layout, Reaction, Delta, Value};
use std::collections::HashMap;

pub fn channel() -> (Sender, Receiver) {
    let (tx, rx) = mpsc::channel(8);
    let sender = Sender { tx };
    let receiver = Receiver { rx };
    (sender, receiver)
}

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<Request>,
}

impl Sender {
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

pub enum Request {
    Subscribe(mpsc::Sender<Response>),
    SetLayout(Layout),
    SetValue(Delta),
}

#[derive(Clone)]
pub enum Response {
    Reaction(Reaction),
}

pub async fn main(mut receiver: Receiver) -> Result<(), Error> {
    let mut subscribers = Vec::new();
    let mut board = HashMap::<Id, Value>::new();
    let mut layout = Layout::Welcome;
    while let Some(request) = receiver.rx.next().await {
        let mut drain_all = false;
        match request {
            Request::Subscribe(mut sender) => {
                // Send layout to a new subscriber
                let reaction = Reaction::Layout(layout.clone());
                let response = Response::Reaction(reaction);
                drain_all |= sender.send(response).await.is_err();
                let snapshot = board.iter().map(|(id, value)| Delta::from((id.clone(), value.clone())));
                for delta in snapshot {
                    let reaction = Reaction::Delta(delta);
                    let response = Response::Reaction(reaction);
                    drain_all |= sender.send(response).await.is_err();
                }
                subscribers.push(sender);
            }
            Request::SetLayout(new_layout) => {
                // Send new_layout to every subscriber
                layout = new_layout;
                let reaction = Reaction::Layout(layout.clone());
                let response = Response::Reaction(reaction);
                for sender in &mut subscribers {
                    drain_all |= sender.send(response.clone()).await.is_err();
                }
            }
            Request::SetValue(delta) => {
                board.insert(delta.id.clone(), delta.value.clone());
                let reaction = Reaction::Delta(delta);
                let mut response = Response::Reaction(reaction);
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
