use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type ControlWidget = WidgetModel<Model>;

pub struct Model {
    control: Option<protocol::Control>,
}

impl Default for Model {
    fn default() -> Self {
        Self { control: None }
    }
}

#[derive(Properties, Default, PartialEq, Clone)]
pub struct Props {
    pub control: Option<protocol::Control>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.control = props.control.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(control) = self.control.as_ref() {
            match control {
                protocol::Control::Button(ref _id) => {
                    html! {
                        <widgets::Button: />
                    }
                }
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}

