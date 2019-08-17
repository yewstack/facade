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
        html! {
            <div class="f-application">
                <div class="f-application--wrap">
                    <div class="f-navigation-drawer">
                        <div class="f-navigation-drawer_border" />
                        <div class="f-navigation-drawer_content">
                            <widgets::List: list=self.app.navigation_drawer.clone() />
                        </div>
                    </div>
                    <div class="app-bar">
                    </div>
                    <div class="content">
                        <widgets::Container: container=self.app.content.clone() />
                    </div>
                </div>
            </div>
        }
    }
}
