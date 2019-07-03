#![feature(async_await)]

mod router;
mod server;

use failure::Error;
use futures3::channel::mpsc;
use futures3::join;

pub async fn main() -> Result<(), Error> {
    let (tx, rx) = router::channel();
    let router = router::main(rx);
    let main = server::main(tx);
    join!(router, main);
    Ok(())
}
