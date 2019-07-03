use failure::Error;
use futures3::channel::mpsc;
use futures3::stream::select;
use futures3::{SinkExt, StreamExt};
use protocol::{Layout, Reaction};

pub enum Request {
    Subscribe(mpsc::Sender<Response>),
    SetLayout(Layout),
}

#[derive(Clone)]
pub enum Response {
    Reaction(Reaction),
}

pub struct Router {
    rx: mpsc::Receiver<Request>,
}

pub async fn router(mut router: Router) -> Result<(), Error> {
    let mut subscribers = Vec::new();
    let mut layout = None;
    while let Some(request) = router.rx.next().await {
        let mut drain_all = false;
        match request {
            Request::Subscribe(mut sender) => {
                // Send layout to a new subscriber
                if let Some(layout) = layout.clone() {
                    let reaction = Reaction::Layout(layout);
                    let response = Response::Reaction(reaction);
                    drain_all |= sender.send(response).await.is_err();
                }
                subscribers.push(sender);
            }
            Request::SetLayout(new_layout) => {
                // Send new_layout to every subscriber
                let reaction = Reaction::Layout(new_layout.clone());
                let response = Response::Reaction(reaction);
                layout = Some(new_layout);
                for sender in &mut subscribers {
                    drain_all |= sender.send(response.clone()).await.is_err();
                }
            }
        }
        if drain_all {
            subscribers.retain(|sender| !sender.is_closed());
        }
    }
    Ok(())
}
