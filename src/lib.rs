#![feature(async_closure)]

mod control;
pub mod dsl;
mod router;
mod server;
mod settings;

pub use control::Control;
use failure::Error;
use futures::channel::mpsc;
use futures::compat::Compat;
use futures::{join, TryFutureExt};
use settings::Settings;
use std::thread;

pub fn main() -> Result<Control, Error> {
    let settings = Settings::parse()?;
    let (tx, rx) = mpsc::channel(8);
    let control = Control::new(tx.clone());
    let tx = router::Sender::wrap(tx);
    let rx = router::Receiver::wrap(rx);
    thread::spawn(move || {
        let fut = routine(settings, tx, rx).map_err(drop);
        tokio::run(Compat::new(Box::pin(fut)));
    });
    Ok(control)
}

async fn routine(
    settings: Settings,
    tx: router::Sender,
    rx: router::Receiver,
) -> Result<(), Error> {
    let router = router::main(rx).map_err(Error::from);
    let main = server::main(settings.clone(), tx).map_err(Error::from);
    let (r1, r2) = join!(router, main);
    r1.and(r2)
}
