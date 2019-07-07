use crate::widgets::{Reqs, View, Widget, WidgetModel};
use protocol::dashboard as frame;
use yew::html;

pub type Dashboard = WidgetModel<Model>;

pub struct Model {
    dashboard: Option<frame::Dashboard>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            dashboard: None,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub dashboard: Option<frame::Dashboard>,
}

impl Widget for Model {
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.dashboard = props.dashboard.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(dashboard) = self.dashboard.as_ref() {
            html! {
                <p>{ &dashboard.title }</p>
            }
        } else {
            html! {
                //<Spinner: />
            }
        }
    }
}
