use crate::router;
use crate::settings::Settings;
use failure::{format_err, Error};
use flate2::read::GzDecoder;
use futures::Stream;
use futures3::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures3::{join, SinkExt, StreamExt, TryFutureExt};
use futures_timer::Interval;
use headers::{ContentType, HeaderMapExt};
use protocol::{Action, OverlayId, Reaction};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use tar::Archive;
use warp::filters::ws::{Message, WebSocket};
use warp::http::{StatusCode, Uri};
use warp::path::Tail;
use warp::reply::Reply;
use warp::Filter;

const DATA: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui.tar.gz"));

pub async fn process_ws(
    settings: Settings,
    mut router: router::Sender,
    websocket: WebSocket,
) -> Result<(), Error> {
    let (tx, rx) = websocket.split();

    let mut router_rx = router.register().await?;
    type Updates = HashMap<OverlayId, Reaction>;
    let throttle_map = Arc::new(Mutex::new(Updates::new()));

    // TODO Read router_rx and send Reactions to a connected client
    let map = throttle_map.clone();
    let outbound_get = (async move || -> Result<(), Error> {
        while let Some(reaction) = router_rx.next().await {
            let mut map = map
                .lock()
                .map_err(|_| format_err!("can't borrow shared map to add a message"))?;
            map.insert(reaction.overlay_id(), reaction);
        }
        Ok(())
    })();

    let map = Arc::downgrade(&throttle_map);
    let outbound_send = (async move || -> Result<(), Error> {
        let mut tx = tx.sink_compat();
        let ms = settings.throttle_ms();
        let mut interval = Interval::new(ms);
        let mut buffer = Vec::new();
        loop {
            interval.next().await;
            if let Some(map) = map.upgrade() {
                let mut map = map
                    .lock()
                    .map_err(|_| format_err!("can't borrow shared map to get a message"))?;
                buffer.extend(map.drain().map(|(_, msg)| msg));
            } else {
                break;
            }
            for reaction in buffer.drain(..) {
                let text = serde_json::to_string(&reaction)?;
                let msg = Message::text(text);
                tx.send(msg).await?;
            }
        }
        Ok(())
    })();

    let inbound = (async move || -> Result<(), Error> {
        let mut rx = rx.compat();
        while let Some(msg) = rx.next().await.transpose()? {
            let text = msg
                .to_str()
                .map_err(|_| format_err!("WebSocket message doesn't contain text"))?;
            let action: Action = serde_json::from_str(text)?;
            log::debug!("Action: {:?}", action);
        }
        Ok(())
    })();
    let (r1, r2, r3) = join!(inbound, outbound_get, outbound_send);
    r1.and(r2).and(r3)
}

pub async fn main(settings: Settings, router: router::Sender) -> Result<(), Error> {
    // TODO Get the full adderss with a single call.
    let port = settings.port();

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
            let settings = settings.clone();
            let router = router.clone();
            ws.on_upgrade(move |websocket| {
                let fut = process_ws(settings, router, websocket).map_err(drop);
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

    warp::serve(routes)
        .bind(([127, 0, 0, 1], port))
        .compat()
        .await
        .or_else(|_| Err(format_err!("server error")))?;
    Ok(())
}
