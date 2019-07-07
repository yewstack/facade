use crate::widgets::{Page, Reqs, Spinner, View, Widget, WidgetModel};
use protocol::dashboard as frame;
use yew::{html, ShouldRender};

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

#[derive(Debug)]
pub enum Msg {
    SelectPage(usize),
}

impl Widget for Model {
    type Message = Msg;
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.dashboard = props.dashboard.to_owned();
        None
    }

    fn handle_inner(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SelectPage(idx) => {
                self.selected_page = idx;
                true
            }
        }
    }

    fn main_view(&self) -> View<Self> {
        if let Some(dashboard) = self.dashboard.as_ref() {
            let page = dashboard.pages.get(self.selected_page).cloned();
            html! {
                <div class="dashboard",>
                    <div class="sidebar",>
                        <div class="header",>
                            <p class="title",>{ &dashboard.title }</p>
                        </div>
                        <ul class="menu",>
                            { for dashboard.pages.iter().enumerate().map(Model::view_page_title) }
                        </ul>
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

impl Model {
    fn view_page_title((idx, page): (usize, &frame::Page)) -> View<Self> {
        html! {
            <li class="item", onclick=|_| Msg::SelectPage(idx).into(),>{ &page.title }</li>
        }
    }
}
