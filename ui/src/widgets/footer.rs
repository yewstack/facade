use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type FooterWidget = WidgetModel<Model>;

pub struct Model {
    footer: protocol::Footer,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub footer: protocol::Footer,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            footer: props.footer.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.footer = props.footer.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut v_footer = vec!["v-footer"];
        v_footer.push("v-footer--fixed");
        v_footer.push("v-sheet");
        v_footer.push("v-sheet--tile");
        v_footer.push("theme--light");
        v_footer.push("indigo");
        let v_footer_style = "left: 0px; right: 0px; bottom: 0px;";
        html! {
            <div class=v_footer style=v_footer_style>
                <span class="white--text">{ "© 2019" }</span>
            </div>
        }
    }
}

