use failure::Error;
use protocol::{Action, Id, Layout, Reaction, Value};
use serde_derive::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use yew::agent::{Agent, AgentLink, Context, HandlerId, Transferable};
use yew::format::Json;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Requirement {
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
    Listen(HashSet<Requirement>),
    Action(Action),
}

impl Transferable for RequestEvt {}

#[derive(Serialize, Deserialize, Debug)]
pub enum ResponseEvt {
    Reaction(Reaction),
}

impl Transferable for ResponseEvt {}

pub struct LiveAgent {
    link: AgentLink<Self>,
    connection: WebSocketTask,
    /// This field keeps all `Requirement` values required by a listener.
    subscriptions: HashMap<HandlerId, HashSet<Requirement>>,
    listeners: HashMap<Requirement, HashSet<HandlerId>>,
    layout: Layout,
    board: HashMap<Id, Value>,
}

impl Agent for LiveAgent {
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
            subscriptions: HashMap::new(),
            listeners: HashMap::new(),
            layout: Layout::Blank,
            board: HashMap::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) {
    }

    fn handle(&mut self, request: Self::Input, who: HandlerId) {
        match request {
            RequestEvt::Listen(new_listen_set) => {
                // It's important to remove all existent values and refresh with new
                let original_set = self.subscriptions.remove(&who);
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
                    self.subscriptions.insert(who, new_listen_set);
}
            }
            RequestEvt::Action(action) => {
                self.send_interaction(action);
            }
        }
    }
}

impl LiveAgent {
    fn send_interaction(&mut self, action: Action) {
        self.connection.send(Json(&action));
    }

    fn send_data_to(&mut self, requirement: Requirement, who: HandlerId) {
        let reaction = {
            match requirement {
                Requirement::LayoutChange => {
                    let layout = self.layout.clone();
                    Some(Reaction::Layout(layout))
                }
                Requirement::AssignUpdate(id) => {
                    self.board.get(&id).cloned()
                        .map(|value| Reaction::Assign { id, value })
                }
            }
        };
        if let Some(reaction) = reaction {
            let response = ResponseEvt::Reaction(reaction);
            self.link.response(who, response);
        }
    }
}
