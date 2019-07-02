use crate::live::{LiveAgent, Requirement, RequestEvt, ResponseEvt};
use std::collections::HashSet;
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, Renderable, ShouldRender};

pub trait Widget: Default + 'static {
    fn requirements(&self) -> HashSet<Requirement>;
    //fn handle_incoming(&mut self, event: ResponseEvt);
    //fn main_view(&self) -> Html<WidgetModel<Self>>;
}

pub struct WidgetModel<T: Widget> {
    connection: Box<dyn Bridge<LiveAgent>>,
    widget: T,
}

pub enum Msg {
    Incoming(ResponseEvt),
}

impl<T: Widget> Component for WidgetModel<T> {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Incoming);
        let connection = LiveAgent::bridge(callback);
        let mut this = Self {
            connection,
            widget: T::default(),
        };
        this.subscribe_updates();
        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Incoming(event) => {
                log::trace!("Incioming event: {:?}", event);
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }
}

impl<T: Widget> Renderable<Self> for WidgetModel<T> {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ "Widget" }</p>
        }
    }
}

impl<T: Widget> WidgetModel<T> {
    fn subscribe_updates(&mut self) {
        let set = self.widget.requirements();
        let request = RequestEvt::Listen(set);
        self.connection.send(request);
    }
}
