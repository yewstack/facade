use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type NavigationDrawerWidget = WidgetModel<Model>;

pub struct Model {
    navigation_drawer: protocol::NavigationDrawer,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub navigation_drawer: protocol::NavigationDrawer,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            navigation_drawer: props.navigation_drawer.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.navigation_drawer = props.navigation_drawer.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut classes = vec!["f-navigation-drawer"];
        classes.push("f-navigation-drawer--fixed");
        let mut drawer_style = String::new();
        drawer_style.push_str("height: 100vh;");
        drawer_style.push_str("top: 0px;");
        drawer_style.push_str("max-height: calc(100% - 36px);");
        drawer_style.push_str("transform: translateX(0%);");
        drawer_style.push_str("width: 256px;");
        html! {
            <div class=classes style=drawer_style>
                <div class="f-navigation-drawer__content">
                    <widgets::List: list=self.navigation_drawer.list.clone()/>
                </div>
                <div class="f-navigation-drawer__border" />
            </div>
        }
    }
}
