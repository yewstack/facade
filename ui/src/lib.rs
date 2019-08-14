#![recursion_limit = "128"]

mod live;
mod utils;
mod widgets;

use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {}

impl Component for Model {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        Self {}
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<Self> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <widgets::Scene: />
        }
    }
}
