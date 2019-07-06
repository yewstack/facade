use crate::router::Request;
use futures3::channel::mpsc;
use futures3::executor::block_on;
use futures3::SinkExt;
use protocol::{Delta, Id, Scene, Value};

#[derive(Clone)]
pub struct Control {
    tx: mpsc::Sender<Request>,
}

impl Control {
    pub(crate) fn new(tx: mpsc::Sender<Request>) -> Self {
        Self { tx }
    }

    pub fn scene(&mut self, scene: impl Into<Scene>) {
        let request = Request::SetScene(scene.into());
        block_on(self.tx.send(request)).expect("RillRate router lost to set scene");
    }

    pub fn assign(&mut self, id: impl Into<Id>, value: impl Into<Value>) {
        let delta = Delta {
            id: id.into(),
            value: value.into(),
        };
        let request = Request::SetValue(delta);
        block_on(self.tx.send(request)).expect("RillRate router lost to set a value");
    }
}
