#![feature(async_await)]

mod control;
mod dsl;
mod router;
mod server;

use control::Control;
pub use dsl::*;
use failure::Error;
use futures3::channel::mpsc;
use futures3::compat::Compat;
use futures3::{join, TryFutureExt};
use std::thread;

pub fn main() -> Control {
    let (tx, rx) = mpsc::channel(8);
    let control = Control::new(tx.clone());
    let tx = router::Sender::wrap(tx);
    let rx = router::Receiver::wrap(rx);
    thread::spawn(move || {
        let fut = routine(tx, rx).map_err(drop);
        tokio::run(Compat::new(Box::pin(fut)));
    });
    control
}

async fn routine(tx: router::Sender, rx: router::Receiver) -> Result<(), Error> {
    let router = router::main(rx);
    let main = server::main(tx);
    let (r1, r2) = join!(router, main);
    r1.and(r2)
}
