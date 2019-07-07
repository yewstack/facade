use crate::widgets::{Page, Reqs, Spinner, View, Widget, WidgetModel};
use protocol::dashboard as frame;
use yew::html;

pub type Dashboard = WidgetModel<Model>;

pub struct Model {
    dashboard: Option<frame::Dashboard>,
    selected_page: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            dashboard: None,
            selected_page: 0,
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
            let page = dashboard.pages.get(self.selected_page).cloned();
            html! {
                <div class="dashboard",>
                    <div class="side-menu",>
                        <div class="header",>
                            <p>{ &dashboard.title }</p>
                        </div>
                    </div>
                    <div class="content",>
                        <Page: page=page, />
                    </div>
                </div>
            }
        } else {
            html! {
                <Spinner: />
            }
        }
    }
}
