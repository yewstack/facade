use crate::widgets::{Reqs, View, Widget, WidgetModel};
use protocol::Value;
use yew::html;

pub type FixedWidget = WidgetModel<Model>;

pub struct Model {
    value: Value,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            value: Value::Nothing,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub value: Value,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.value = props.value.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <p class="fixed",>{ &self.value }</p>
        }
    }
}
