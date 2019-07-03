use crate::router;
use failure::{format_err, Error};
use flate2::read::GzDecoder;
use futures::Stream;
use futures3::channel::mpsc;
use futures3::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures3::{SinkExt, StreamExt, TryFutureExt};
use headers::{ContentType, HeaderMapExt};
use protocol::{Action, Layout, Reaction};
use std::collections::HashMap;
use std::env;
use std::io::Read;
use tar::Archive;
use warp::filters::ws::{Message, WebSocket};
use warp::http::{StatusCode, Uri};
use warp::path::Tail;
use warp::reply::Reply;
use warp::Filter;

const PORT_VAR: &str = "RILLRATE_PORT";
const PORT_DEF: &str = "12400";
const DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui.tar.gz"));

pub async fn process_ws(router: router::Sender, websocket: WebSocket) -> Result<(), Error> {
    let (tx, rx) = websocket.split();

    // TODO Register itself to router

    let mut tx = tx.sink_compat();
    let notification = Reaction::Layout(Layout::Blank);
    let text = serde_json::to_string(&notification)?;
    let msg = Message::text(text);
    tx.send(msg).await?;

    let mut rx = rx.compat();
    while let Some(msg) = rx.next().await.transpose()? {
        let text = msg
            .to_str()
            .map_err(|_| format_err!("WebSocket message doesn't contain text"))?;
        let action: Action = serde_json::from_str(text)?;
        log::debug!("Action: {:?}", action);
    }
    Ok(())
}

pub async fn main(router: router::Sender) -> Result<(), Error> {
    let tar = GzDecoder::new(DATA);
    let mut archive = Archive::new(tar);
    let mut files = HashMap::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        let mut data = Vec::new();
        entry.read_to_end(&mut data)?;
        if data.len() > 0 {
            let name = entry
                .path()?
                .to_str()
                .map(|s| &s[2..])
                .ok_or_else(|| format_err!("can't get path from static srchaive"))?
                .to_owned();
            log::trace!("Register asset file: {}", name);
            files.insert(name, data);
        }
    }

    let index = warp::path::end().map(|| warp::redirect(Uri::from_static("/index.html")));

    let live = warp::path("live")
        .and(warp::ws2())
        .map(move |ws: warp::ws::Ws2| {
            let router = router.clone();
            ws.on_upgrade(move |websocket| {
                let fut = process_ws(router, websocket).map_err(drop);
                Compat::new(Box::pin(fut))
            })
        });

    let assets = warp::path::tail().map(move |tail: Tail| {
        log::trace!("req: {}", tail.as_str());
        let mime = mime_guess::guess_mime_type(tail.as_str());
        let mut resp = files
            .get(tail.as_str())
            .map(|data| data.clone().into_response())
            .unwrap_or_else(|| StatusCode::NOT_FOUND.into_response());
        resp.headers_mut().typed_insert(ContentType::from(mime));
        resp
    });

    let routes = index.or(live).or(assets);

    let port: u16 = env::var(PORT_VAR).unwrap_or(PORT_DEF.to_string()).parse()?;
    warp::serve(routes)
        .bind(([127, 0, 0, 1], port))
        .compat()
        .await
        .or_else(|_| Err(format_err!("server error")))?;
    Ok(())
}
