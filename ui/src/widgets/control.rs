use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type ControlWidget = WidgetModel<Model>;

pub struct Model {
    control: protocol::Control,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub control: protocol::Control,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self { control: props.control.to_owned() }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.control = props.control.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        match self.control {
            protocol::Control::Button(ref _id) => {
                html! {
                    <widgets::Button: />
                }
            }
        }
    }
}

