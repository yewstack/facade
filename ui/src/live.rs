use failure::Error;
use protocol::{Action, Id, Reaction};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use yew::agent::{Agent, AgentLink, Context, HandlerId, Transferable};
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Interest {
    LayoutChange,
    AssignUpdate(Id),
}

pub enum Msg {
    Received(Result<Reaction, Error>),
    StatusChanged(WebSocketStatus),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RequestEvt {
    /// Pass empty `HashSet` to unsubscribe.
    Listen(HashSet<Interest>),
    Action(Action),
}

impl Transferable for RequestEvt {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseEvt {
    Reaction(Reaction),
}

impl Transferable for ResponseEvt {}

pub struct Live {
    link: AgentLink<Self>,
    connection: WebSocketTask,
    /// This field keeps all `Interest` values required by a listener.
    interests: HashMap<HandlerId, HashSet<Interest>>,
    listeners: HashMap<Interest, HashSet<HandlerId>>,
}

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
            interests: HashMap::new(),
            listeners: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        match request {
            RequestEvt::Listen(new_listen_set) => {
                // It's important to remove all existent values and refresh with new
                let original_set = self.interests.remove(&who);
                if let Some(to_remove) = original_set {
                    // Unsubscribe all, because if a client subscribes again
                    // we have to resend all updates again.
                    for requirement in to_remove {
                        log::trace!("Unsubscribed from: {:?}", requirement);
                        self.listeners
                            .entry(requirement.clone())
                            .or_default()
                            .remove(&who);
                    }
                }
                if !new_listen_set.is_empty() {
                    // or unsubscribe only if empty
                    for requirement in &new_listen_set {
                        log::trace!("Subscribed to: {:?}", requirement);
                        self.listeners
                            .entry(requirement.clone())
                            .or_default()
                            .insert(who);
                        self.send_data_to(requirement.clone(), who);
                    }
                    self.interests.insert(who, new_listen_set);
}
            }
            RequestEvt::Action(action) => {
                self.send_interaction(action);
            }
        }
    }
}

impl Live {
    fn send_interaction(&mut self, action: Action) {
        self.connection.send(Json(&action));
    }

    fn send_data_to(&mut self, interest: Interest, who: HandlerId) {
        // TODO: Send actual values from a data-board
    }
}
