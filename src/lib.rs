#![feature(async_await)]

use failure::Error;
use futures3::compat::{Future01CompatExt};//, Sink01CompatExt, Stream01CompatExt};
use hyper::{Body, Response, Server};
use hyper::service::service_fn_ok;
use std::env;

const PORT_VAR: &str = "RILLRATE_PORT";
const PORT_DEF: &str = "12400";

pub async fn main() -> Result<(), Error> {
    let port: u16 = env::var(PORT_VAR).unwrap_or(PORT_DEF.to_string()).parse()?;
    let addr = ([127, 0, 0, 1], port).into();
    let make_service = || {
        service_fn_ok(|_req| {
            Response::new(Body::from("Hello World"))
        })
    };
    let server = Server::bind(&addr)
        .serve(make_service)
        .compat();
    server.await?;
    Ok(())
}
