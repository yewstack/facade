#![feature(async_await)]

use failure::Error;
use futures::compat::Future01CompatExt;
use futures::future::select;
use futures::{FutureExt, StreamExt};
use futures_legacy::{Future as _, Stream as _};

#[runtime::main(runtime_tokio::Tokio)]
pub async fn main() -> Result<(), Error> {
    env_logger::try_init()?;
    let ctrl_c = tokio_signal::ctrl_c()
        .flatten_stream()
        .into_future()
        .compat();

    let mut control = facade::main()?;
    let scene = {
        use facade::dsl::*;
        Scene(App(
            NavigationDrawer(
                List(vec![
                    ListItem(Icon::Home, "MenuItem".into()),
                ])
            ),
            Container(
                Layout(vec![
                   Flex(),
                   Flex(),
                   Flex(),
                ])
            )
        ))
    };
    control.scene(scene);

    ctrl_c.await;
    Ok(())
}
