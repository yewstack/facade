use crate::utils::ToClass;
use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::{Col, Component, Row};
use yew::{html, Properties};

pub type RowWidget = WidgetModel<Model>;

pub struct Model {
    row: Row,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub row: Row,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self {
            row: props.row.clone(),
        }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.row = props.row.clone();
        None
    }

    fn main_view(&self) -> View<Self> {
        let mut classes = Vec::with_capacity(10);
        classes.push("row");
        if self.row.wrap {
            classes.push("wrap");
        }
        if self.row.fill {
            classes.push("fill");
        }
        if self.row.reverse {
            classes.push("reverse");
        }
        if let Some(ref direction) = self.row.direction {
            classes.push(direction.to_class());
        }
        if let Some(ref align) = self.row.align {
            classes.push(align.to_class());
        }
        if let Some(ref justify) = self.row.justify {
            classes.push(justify.to_class());
        }
        html! {
            <div class=classes>
                { for self.row.cols.iter().map(|col| self.view_col(col)) }
            </div>
        }
    }
}

impl Model {
    fn view_col(&self, col: &Col) -> View<Self> {
        html! {
            let mut classes = Vec::with_capacity(10);
            classes.push("col");
            html! {
                <div class=classes>
                    { for col.components.iter().map(|comp| self.view_comp(comp)) }
                </div>
            }
        }
    }

    fn view_comp(&self, comp: &Component) -> View<Self> {
        html! {
            <widgets::Component: component=comp.clone() />
        }
    }
}
