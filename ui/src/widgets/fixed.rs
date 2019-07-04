use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type Fixed = WidgetModel<Model>;

pub struct Model {}

impl Default for Model {
    fn default() -> Self {
        Self {}
    }
}

impl Widget for Model {
    fn main_view(&self) -> View<Self> {
        html! {
            <p>{ "Fixed!" }</p>
        }
    }
}
