use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type AppBarWidget = WidgetModel<Model>;

pub struct Model {
    app_bar: protocol::Bar,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub app_bar: protocol::Bar,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            app_bar: props.app_bar.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.app_bar = props.app_bar.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut v_app_bar = vec!["v-app-bar"];
        v_app_bar.push("v-app-bar--fixed");
        v_app_bar.push("v-sheet");
        v_app_bar.push("v-sheet--tile");
        // TODO: Changeable
        v_app_bar.push("theme--dark");
        v_app_bar.push("v-toolbar");
        // TODO: Changeable
        v_app_bar.push("indigo");

        let mut app_bar_style = String::new();
        app_bar_style.push_str("margin-top: 0px;");
        app_bar_style.push_str("transform: translateY(0px);");
        app_bar_style.push_str("left: 256px;");
        app_bar_style.push_str("right: 0px;");

        html! {
            <div class=v_app_bar style=app_bar_style>
                <div class="v-toolbar__content" style="height: 64px;">
                    <div class="v-app-bar__nav-icon" />
                    <div class="v-toolbar__title">{ &self.app_bar.title.caption }</div>
                </div>
            </div>
        }
    }
}
