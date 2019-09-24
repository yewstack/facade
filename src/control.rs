use crate::router::Request;
use failure::Error;
use futures::channel::mpsc;
use futures::SinkExt;
use protocol::{Delta, Id, Scene, Value};
use std::fmt;

#[derive(Clone)]
pub struct Control {
    tx: mpsc::Sender<Request>,
}

impl fmt::Debug for Control {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Control").finish()
    }
}

impl Control {
    pub(crate) fn new(tx: mpsc::Sender<Request>) -> Self {
        Self { tx }
    }

    async fn send_request(&mut self, request: Request) -> Result<(), Error> {
        self.tx.send(request).await.map_err(Error::from)
    }

    pub async fn scene(&mut self, scene: impl Into<Scene>) -> Result<(), Error> {
        let request = Request::SetScene(scene.into());
        self.send_request(request).await
    }

    pub async fn assign(&mut self, id: impl Into<Id>, value: impl Into<Value>) -> Result<(), Error> {
        let delta = Delta {
            id: id.into(),
            value: value.into(),
        };
        let request = Request::SetValue(delta);
        self.send_request(request).await
    }
}
