use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::html;

pub type ListWidget = WidgetModel<Model>;

pub struct Model {
    list: Option<protocol::List>,
}

impl Default for Model {
    fn default() -> Self {
        Self { list: None }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub list: Option<protocol::List>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

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
                <div class="list-item-value",>{ "-" }</div>
            </div>
        }
    }
}
