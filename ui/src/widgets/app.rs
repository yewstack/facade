use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::App;
use yew::{html, Properties};

pub type AppWidget = WidgetModel<Model>;

pub struct Model {
    app: App,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub app: App,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            app: props.app.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.app = props.app.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut classes = vec!["app"];
        html! {
            <div class=classes>
                <div class="navigation-drawer">
                    <widgets::List: list=self.app.navigation_drawer.clone() />
                </div>
                <div class="app-bar">
                </div>
                <div class="content">
                    <widgets::Container: container=self.app.content.clone() />
                </div>
            </div>
        }
    }
}
