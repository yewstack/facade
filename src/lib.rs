#![feature(async_await)]

mod router;
mod server;

use failure::Error;
use futures3::channel::mpsc;
use futures3::compat::Compat;
use futures3::executor::block_on;
use futures3::{join, SinkExt, StreamExt, TryFutureExt};
use protocol::{Delta, Id, Layout, Value};
use std::thread;

pub fn main() -> Control {
    let (tx, rx) = mpsc::channel(8);
    let control = Control { tx: tx.clone() };
    let tx = router::Sender::wrap(tx);
    let rx = router::Receiver::wrap(rx);
    thread::spawn(move || {
        let fut = routine(tx, rx).map_err(drop);
        tokio::run(Compat::new(Box::pin(fut)));
    });
    control
}

async fn routine(tx: router::Sender, rx: router::Receiver)
    -> Result<(), Error>
{
    let router = router::main(rx);
    let main = server::main(tx);
    join!(router, main);
    Ok(())
}

#[derive(Clone)]
pub struct Control {
    tx: mpsc::Sender<router::Request>,
}

impl Control {
    pub fn layout(&mut self, layout: Layout) {
        let request = router::Request::SetLayout(layout);
        block_on(self.tx.send(request))
            .expect("RillRate router lost to set layout");
    }

    pub fn assign(&mut self, id: impl Into<Id>, value: impl Into<Value>) {
        let delta = Delta {
            id: id.into(),
            value: value.into(),
        };
        let request = router::Request::SetValue(delta);
        block_on(self.tx.send(request))
            .expect("RillRate router lost to set a value");
    }
}

