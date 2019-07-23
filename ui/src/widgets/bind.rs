use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type BindWidget = WidgetModel<Model>;

pub struct Model {
    bind: protocol::Bind,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub bind: protocol::Bind,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            bind: props.bind.to_owned(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.bind = props.bind.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        match self.bind {
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
    }
}
