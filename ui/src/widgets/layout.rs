use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Layout;
use yew::html;

pub type LayoutWidget = WidgetModel<Model>;

pub struct Model {
    layout: Layout,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            layout: Layout::Blank,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub layout: Option<Layout>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        if let Some(ref layout) = props.layout {
            self.layout = layout.clone();
        }
        None
    }

    fn main_view(&self) -> View<Self> {
        match self.layout {
            Layout::Blank => {
                html! {
                    <p>{ "Blank" }</p>
                }
            }
            Layout::Welcome => {
                html! {
                    <p>{ "Welcome" }</p>
                }
            }
            Layout::Bind(ref bind) => {
                html! {
                    <widgets::Bind: bind = Some(bind.clone()), />
                }
            }
            Layout::Control(ref control) => {
                html! {
                    <widgets::Control: control = Some(control.clone()), />
                }
            }
            Layout::Row(ref layouts) => {
                html! {
                    <div class="layout-row",>
                        { for layouts.iter().map(|lyo| self.row(lyo)) }
                    </div>
                }
            }
            Layout::Column(ref layouts) => {
                html! {
                    <div class="layout-column",>
                        { for layouts.iter().map(|lyo| self.column(lyo)) }
                    </div>
                }
            }
            Layout::List(ref list) => {
                html! {
                    <widgets::List: list = Some(list.clone()), />
                }
            }
            Layout::Container(ref container) => {
                html! {
                    <widgets::Container: container = Some(*container.clone()), />
                }
            }
        }
    }
}

impl Model {
    fn column(&self, layout: &Layout) -> View<Self> {
        html! {
            <widgets::Layout: layout=Some(layout.clone()), />
        }
    }

    fn row(&self, layout: &Layout) -> View<Self> {
        html! {
            <widgets::Layout: layout=Some(layout.clone()), />
        }
    }
}
