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
        let mut v_application = vec!["v-application"];
        // TODO: Check `rtl` flag in app
        v_application.push("v-application--is-ltr");
        // TODO: Check `dark` flag in app
        v_application.push("theme--light");

        let mut app_bar_style = String::new();
        app_bar_style.push_str("margin-top: 0px;");
        app_bar_style.push_str("transform: translateY(0px);");
        app_bar_style.push_str("left: 256px;");
        app_bar_style.push_str("right: 0px;");

        html! {
            <div class=v_application>
                <div class="v-application--wrap">
                    <widgets::NavigationDrawer: navigation_drawer=self.app.navigation_drawer.clone() />
                    <div class="v-app-bar v-app-bar--fixed v-toolbar" style=app_bar_style>
                        <div class="v-toolbar__content">
                            <div class="v-app-bar__nav-icon" />
                            <div class="v-toolbar__title">{ &self.app.app_bar.title.caption }</div>
                        </div>
                    </div>
                    <div class="v-content">
                        <widgets::Container: container=self.app.content.clone() />
                    </div>
                    <div class="v-footer">
                    </div>
                </div>
            </div>
        }
    }
}
