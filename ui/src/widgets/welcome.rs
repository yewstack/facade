use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type WelcomeWidget = WidgetModel<Model>;

pub struct Model {}

impl Widget for Model {
    type Message = ();
    type Properties = ();

    fn produce(props: &Self::Properties) -> Self {
        Self { }
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <p>{ "Welcome!" }</p>
        }
    }
}
