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

        let v_content_style = "padding: 64px 0px 36px 256px;";

        html! {
            <div class=v_application>
                <div class="v-application--wrap">
                    <widgets::NavigationDrawer: navigation_drawer=self.app.navigation_drawer.clone() />
                    <widgets::AppBar: app_bar=self.app.app_bar.clone() />
                    <div class="v-content" style=v_content_style>
                        <div class="v-content__wrap">
                            <widgets::Container: container=self.app.content.clone() />
                        </div>
                    </div>
                    <widgets::Footer: footer=self.app.footer.clone() />
                </div>
            </div>
        }
    }
}
