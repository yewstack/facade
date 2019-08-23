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
        let mut v_icon = vec!["v-icon"];
        v_icon.push("notranslate");
        v_icon.push("material-icons");
        v_icon.push("theme--light");
        html! {
            <i class=v_icon>{ self.to_class() }</i>
        }
    }

    fn to_class(&self) -> &'static str {
        use protocol::Icon::*;
        match self.icon {
            Home => "home",
            ContactMail => "contact_mail",
            MenuSandwich => "_change it in Rust source",
        }
    }
}
