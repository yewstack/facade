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
        let mut app_bar_style = String::new();
        app_bar_style.push_str("margin-top: 0px;");
        app_bar_style.push_str("transform: translateY(0px);");
        app_bar_style.push_str("left: 256px;");
        app_bar_style.push_str("right: 0px;");
        html! {
            <div class="f-application">
                <div class="f-application--wrap">
                    <widgets::NavigationDrawer: navigation_drawer=self.app.navigation_drawer.clone() />
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
