use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::html;

pub type BindWidget = WidgetModel<Model>;

pub struct Model {
    bind: Option<protocol::Bind>,
}

impl Default for Model {
    fn default() -> Self {
        Self { bind: None }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub bind: Option<protocol::Bind>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.bind = props.bind.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(bind) = self.bind.as_ref() {
            match bind {
                protocol::Bind::Fixed(ref value) => {
                    html! {
                        <widgets::Fixed: value=value, />
                    }
                }
                protocol::Bind::Dynamic(ref id) => {
                    html! {
                        <widgets::Dynamic: id=id, />
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
