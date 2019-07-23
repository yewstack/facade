use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type ListWidget = WidgetModel<Model>;

pub struct Model {
    list: Option<protocol::List>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub list: Option<protocol::List>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self { list: None }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.list = props.list.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(list) = self.list.as_ref() {
            html! {
                <div class="list",>
                    { for list.items.iter().map(|item| self.view_item(item)) }
                </div>
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}

impl Model {
    fn view_item(&self, item: &protocol::ListItem) -> View<Self> {
        html! {
            <div class="list-item",>
                <div class="list-item-info",>
                    <div class="list-item-info-title",>{ &item.title }</div>
                    <div class="list-item-info-description",>{ &item.description }</div>
                </div>
                <div class="list-item-value",>
                    <widgets::Bind: bind=item.bind.clone(), />
                </div>
            </div>
        }
    }
}
