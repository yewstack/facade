use yew::html::Component;
use yew::{html, Html};

pub struct Icon {
    icon: protocol::Icon,
}

impl Icon {
    pub fn new(icon: protocol::Icon) -> Self {
        Self { icon }
    }

    pub fn render<T: Component>(&self) -> Html<T> {
        html! {
            <p>{ "Icon"}</p>
        }
    }
}
