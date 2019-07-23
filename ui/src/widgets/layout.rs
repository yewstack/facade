use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Layout;
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
                    <widgets::Bind: bind = bind.clone(), />
                }
            }
            Layout::Control(ref control) => {
                html! {
                    <widgets::Control: control = control.clone(), />
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
                    <widgets::List: list = list.clone(), />
                }
            }
            Layout::Container(ref container) => {
                html! {
                    <widgets::Container: container = *container.clone(), />
                }
            }
        }
    }
}

impl Model {
    fn column(&self, layout: &Layout) -> View<Self> {
        html! {
            <widgets::Layout: layout=layout.clone(), />
        }
    }

    fn row(&self, layout: &Layout) -> View<Self> {
        html! {
            <widgets::Layout: layout=layout.clone(), />
        }
    }
}
