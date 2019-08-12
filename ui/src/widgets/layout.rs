use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use crate::utils::ToClass;
use protocol::{Flex, Layout};
use yew::{html, Properties};

pub type LayoutWidget = WidgetModel<Model>;

pub struct Model {
    layout: Layout,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub layout: Layout,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            layout: props.layout.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.layout = props.layout.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut classes = Vec::with_capacity(10);
        classes.push("layout");
        if self.layout.wrap {
            classes.push("wrap");
        }
        if self.layout.fill {
            classes.push("fill");
        }
        if self.layout.reverse {
            classes.push("reverse");
        }
        if let Some(ref direction) = self.layout.direction {
            classes.push(direction.to_class());
        }
        if let Some(ref align) = self.layout.align {
            classes.push(align.to_class());
        }
        if let Some(ref justify) = self.layout.justify {
            classes.push(justify.to_class());
        }
        html! {
            <div class=classes>
                { for self.layout.flex_vec.iter().map(|flex| self.view_flex(flex)) }
            </div>
        }
    }
}

impl Model {
    fn view_flex(&self, flex: &Flex) -> View<Self> {
        html! {
            let mut classes = Vec::with_capacity(10);
            classes.push("flex");
            html! {
                <div class=classes>
                </div>
            }
        }
    }
}
