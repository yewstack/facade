use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::dashboard;
use yew::{html, Properties};

pub type PageWidget = WidgetModel<Model>;

pub struct Model {
    page: dashboard::Page,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub page: dashboard::Page,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            page: props.page.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.page = props.page.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        html! {
            <div class="page",>
                <div class="header",>
                    <p class="title",>{ &self.page.title }</p>
                    <div class="separator",></div>
                    <p class="subtitle",>{ &self.page.subtitle }</p>
                </div>
                <div class="body",>
                    <widgets::Layout: layout=self.page.body.clone(), />
                </div>
            </div>
        }
    }
}
