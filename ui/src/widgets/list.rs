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
        let mut v_list = vec!["v-list"];
        v_list.push("v-sheet");
        v_list.push("v-sheet--tile");
        // TODO: How to get theme parameter here?
        v_list.push("theme--light");
        if self.list.dense {
            v_list.push("v-list--dense");
        }
        html! {
            <div class=v_list,>
                { for self.list.items.iter().map(|item| self.view_item(item)) }
            </div>
        }
    }
}

impl Model {
    fn view_item(&self, item: &protocol::ListItem) -> View<Self> {
        let mut v_list_item = vec!["v-list-item"];
        v_list_item.push("v-list-item--link");
        v_list_item.push("theme--light");
        html! {
            <div class=v_list_item>
                <div class="v-list-item__action">
                    { widgets::Icon::new(item.action.clone()).render() }
                </div>
                <div class="v-list-item__content">
                    <div class="v-list-item__title">
                        { &item.content.caption }
                    </div>
                </div>
            </div>
        }
    }
}
