use crate::router::Request;
use futures3::channel::mpsc;
use futures3::executor::block_on;
use futures3::SinkExt;
use protocol::{Delta, Id, Layout, Value};

#[derive(Clone)]
pub struct Control {
    tx: mpsc::Sender<Request>,
}

impl Control {
    pub(crate) fn new(tx: mpsc::Sender<Request>) -> Self {
        Self { tx }
    }

    pub fn layout(&mut self, layout: Layout) {
        let request = Request::SetLayout(layout);
        block_on(self.tx.send(request)).expect("RillRate router lost to set layout");
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
