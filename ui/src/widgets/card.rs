use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Card;
use yew::{html, Properties};

pub type CardWidget = WidgetModel<Model>;

pub struct Model {
    card: Card,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub card: Card,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            card: props.card.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.card = props.card.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut v_card = vec!["v-card"];
        html! {
            <div class=v_card>
            </div>
        }
    }
}

