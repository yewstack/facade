use crate::live::{LiveAgent, RequestEvt, Requirement, ResponseEvt};
use std::collections::HashSet;
use yew::{Bridge, Bridged, Component, ComponentLink, Html, Renderable, ShouldRender};

pub type Reqs = Option<HashSet<Requirement>>;
pub type View<T> = Html<WidgetModel<T>>;

pub trait Widget: Default + 'static {
    type Properties: Default + Clone + PartialEq;

    fn recompose(&mut self, _props: &Self::Properties) -> Reqs {
        None
    }

    fn handle_incoming(&mut self, _event: ResponseEvt) -> ShouldRender {
        false
    }

    fn main_view(&self) -> View<Self>;
}

pub struct WidgetModel<T: Widget> {
    connection: Box<dyn Bridge<LiveAgent>>,
    widget: T,
    props: T::Properties,
    requirements: HashSet<Requirement>,
}

pub enum Msg {
    Incoming(ResponseEvt),
}

impl<T: Widget> Component for WidgetModel<T> {
    type Message = Msg;
    type Properties = T::Properties;

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Incoming);
        let connection = LiveAgent::bridge(callback);
        let mut this = Self {
            connection,
            widget: T::default(),
            props,
            requirements: HashSet::new(),
        };
        this.recompose_inner_component();
        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Incoming(event) => {
                log::trace!("Incioming event: {:?}", event);
                self.widget.handle_incoming(event)
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        self.recompose_inner_component();
        true
    }
}

impl<T: Widget> Renderable<Self> for WidgetModel<T> {
    fn view(&self) -> Html<Self> {
        self.widget.main_view()
    }
}

impl<T: Widget> WidgetModel<T> {
    fn recompose_inner_component(&mut self) {
        if let Some(new_requirements) = self.widget.recompose(&self.props) {
            if self.requirements != new_requirements {
                self.requirements = new_requirements;
                let set = self.requirements.clone();
                let request = RequestEvt::Listen(set);
                self.connection.send(request);
            }
        }
    }
}
