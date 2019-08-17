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
        let mut drawer_style = String::new();
        drawer_style.push_str("height: 100vh;");
        drawer_style.push_str("top: 0px;");
        drawer_style.push_str("max-height: calc(100% - 36px);");
        drawer_style.push_str("transform: translateX(0%);");
        drawer_style.push_str("width: 256px;");
        let mut app_bar_style = String::new();
        app_bar_style.push_str("margin-top: 0px;");
        app_bar_style.push_str("transform: translateY(0px);");
        app_bar_style.push_str("left: 256px;");
        app_bar_style.push_str("right: 0px;");
        html! {
            <div class="f-application">
                <div class="f-application--wrap">
                    <div class="f-navigation-drawer f-navigation-drawer--fixed" style=drawer_style>
                        <div class="f-navigation-drawer__content">
                            <widgets::List: list=self.app.navigation_drawer.clone() />
                        </div>
                        <div class="f-navigation-drawer__border" />
                    </div>
                    <div class="f-app-bar f-toolbar" style=app_bar_style>
                    </div>
                    <div class="f-content">
                        <widgets::Container: container=self.app.content.clone() />
                    </div>
                    <div class="f-footer">
                    </div>
                </div>
            </div>
        }
    }
}
