use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type WelcomeWidget = WidgetModel<Model>;

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for Model {
    type Message = ();
    type Properties = ();

    fn main_view(&self) -> View<Self> {
        html! {
            <p>{ "Welcome!" }</p>
        }
    }
}
