use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type ListWidget = WidgetModel<Model>;

pub struct Model {
    list: protocol::List,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub list: protocol::List,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            list: props.list.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.list = props.list.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut classes = vec!["list"];
        if self.list.dense {
            classes.push("dense");
        }
        html! {
            <div class=classes,>
                { for self.list.items.iter().map(|item| self.view_item(item)) }
            </div>
        }
    }
}

impl Model {
    fn view_item(&self, item: &protocol::ListItem) -> View<Self> {
        html! {
            <div class="list-item",>
                <div class="list-item-action">
                    { widgets::Icon::new(item.action.clone()).render() }
                </div>
                <div class="list-item-content">
                    { &item.content.caption }
                </div>
            </div>
        }
    }
}
