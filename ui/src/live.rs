use failure::Error;
use protocol::{Action, Reaction};
use serde_derive::{Deserialize, Serialize};
use yew::agent::{Agent, AgentLink, Context, HandlerId, Transferable};
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};


pub struct Live {
    link: AgentLink<Self>,
    connection: WebSocketTask,
}

pub enum Msg {
    Received(Result<Reaction, Error>),
    StatusChanged(WebSocketStatus),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestEvt {
}

impl Transferable for RequestEvt {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseEvt {
}

impl Transferable for ResponseEvt {}

impl Agent for Live {
    type Reach = Context;
    type Message = Msg;
    type Input = RequestEvt;
    type Output = ResponseEvt;

    fn create(link: AgentLink<Self>) -> Self {
        let messages = link.send_back(|Json(data)| Msg::Received(data));
        let notifications = link.send_back(Msg::StatusChanged);
        let mut ws = WebSocketService::new();
        let host = yew::utils::host().unwrap();
        let path = format!("ws://{}/live/", host);
        log::trace!("WS: {}", path);
        let connection = ws.connect(&path, messages, notifications);
        Self {
            link,
            connection,
        }
    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
    }
}
