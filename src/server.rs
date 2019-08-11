use crate::router;
use crate::settings::Settings;
use failure::Fail;
use flate2::read::GzDecoder;
use futures::compat::{Compat, Future01CompatExt, Sink01CompatExt, Stream01CompatExt};
use futures::{join, SinkExt, StreamExt, TryFutureExt};
use futures_legacy::Stream;
use futures_timer::Interval;
use headers::{ContentType, HeaderMapExt};
use protocol::{Action, Message as _, OverlayId, Reaction};
use std::collections::HashMap;
use std::io::Read;
use std::sync::{Arc, Mutex};
use tar::Archive;
use warp::filters::ws::{Message, WebSocket};
use warp::http::{StatusCode, Uri};
use warp::path::Tail;
use warp::reply::Reply;
use warp::Filter;

const ASSETS: &'static [u8] = include_bytes!(concat!(env!("OUT_DIR"), "/ui.tar.gz"));

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "io error: {}", _0)]
    IoError(#[cause] std::io::Error),
    #[fail(display = "protocol error: {}", _0)]
    ProtocolError(#[cause] protocol::Error),
    #[fail(display = "wrap error: {}", _0)]
    WarpError(#[cause] warp::Error),
    #[fail(display = "router error: {}", _0)]
    RouterError(#[cause] router::Error),
    #[fail(display = "can't lock throttle map")]
    CantLockThrottleMap,
    #[fail(display = "wrong assets format")]
    WrongAssetsFormat,
    #[fail(display = "bind error")]
    BindError,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<warp::Error> for Error {
    fn from(err: warp::Error) -> Self {
        Error::WarpError(err)
    }
}

impl From<router::Error> for Error {
    fn from(err: router::Error) -> Self {
        Error::RouterError(err)
    }
}

impl From<protocol::Error> for Error {
    fn from(err: protocol::Error) -> Self {
        Error::ProtocolError(err)
    }
}

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
            let mut map = map.lock().map_err(|_| Error::CantLockThrottleMap)?;
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
                let mut map = map.lock().map_err(|_| Error::CantLockThrottleMap)?;
                buffer.extend(map.drain().map(|(_, msg)| msg));
            } else {
                break;
            }
            for reaction in buffer.drain(..) {
                let payload = reaction.serialize()?;
                let msg = Message::binary(payload);
                tx.send(msg).await?;
            }
        }
        Ok(())
    })();

    let inbound = (async move || -> Result<(), Error> {
        let mut rx = rx.compat();
        while let Some(msg) = rx.next().await.transpose()? {
            let payload = msg.as_bytes();
            let action = Action::deserialize(payload)?;
            log::debug!("Action: {:?}", action);
        }
        Ok(())
    })();
    let (r1, r2, r3) = join!(inbound, outbound_get, outbound_send);
    r1.and(r2).and(r3)
}

pub async fn main(settings: Settings, router: router::Sender) -> Result<(), Error> {
    let address = settings.socket_addr();

    let tar = GzDecoder::new(ASSETS);
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
                .ok_or_else(|| Error::WrongAssetsFormat)?
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
        .bind(address)
        .compat()
        .await
        .or_else(|_| Err(Error::BindError))?;
    Ok(())
}
