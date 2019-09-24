use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Component;
use yew::{html, Properties};

pub type ComponentWidget = WidgetModel<Model>;

pub struct Model {
    component: Component,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub component: Component,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            component: props.component.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.component = props.component.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <p>{ "Component" }</p>
        }
    }
}
