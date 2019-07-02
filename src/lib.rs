#![feature(async_await)]

use failure::{format_err, Error};
use flate2::read::GzDecoder;
use futures::{Future, Stream};
use futures3::{StreamExt, TryFutureExt};
use futures3::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use std::env;
use std::io::Read;
use std::collections::HashMap;
use tar::Archive;
use warp::Filter;
use warp::filters::ws::WebSocket;

const PORT_VAR: &str = "RILLRATE_PORT";
const PORT_DEF: &str = "12400";
const DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui.tar.gz"));

pub async fn process_ws(websocket: WebSocket) -> Result<(), Error> {
    let (tx, rx) = websocket.split();
    let tx = tx.sink_compat();
    let mut rx = rx.compat();
    while let Some(msg) = rx.next().await {
    }
    Ok(())
}

pub async fn main() -> Result<(), Error> {
    let mut tar = GzDecoder::new(DATA);
    let mut archive = Archive::new(tar);
    let mut files = HashMap::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        let mut data = Vec::new();
        if data.len() > 0 {
            entry.read_to_end(&mut data)?;
            let name = entry
                .path()?
                .to_str()
                .ok_or_else(|| format_err!("can't get path from static srchaive"))?
                .to_owned();
            files.insert(name, data);
        }
    }

    let live = warp::path("live")
        .and(warp::ws2())
        .map(|ws: warp::ws::Ws2| {
            ws.on_upgrade(|websocket| {
                let fut = process_ws(websocket).map_err(drop);
                Compat::new(Box::pin(fut))
            })
        });
    let index = warp::path::end().map(|| warp::reply::html("RillRate"));
    let routes = index.or(live);
    let port: u16 = env::var(PORT_VAR).unwrap_or(PORT_DEF.to_string()).parse()?;
    warp::serve(routes).bind(([127, 0, 0, 1], port)).compat().await;
    Ok(())
}
