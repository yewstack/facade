use crate::live::{Requirement, ResponseEvt};
use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::{Layout, Reaction};
use yew::{html, ShouldRender};

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
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        if let Some(ref layout) = props.layout {
            self.layout = layout.clone();
            // Don't subscribe if layout was set by properties
            None
        } else {
            Some(vec![Requirement::LayoutChange].into_iter().collect())
        }
    }

    fn handle_incoming(&mut self, event: ResponseEvt) -> ShouldRender {
        if let ResponseEvt::Reaction(Reaction::Layout(layout)) = event {
            log::info!("Changing layout: {:?}", layout);
            self.layout = layout;
            true
        } else {
            false
        }
    }

    fn main_view(&self) -> View<Self> {
        use protocol::Widget;
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
            Layout::Widget(ref widget) => match widget {
                Widget::Dynamic(id) => {
                    html! {
                        <widgets::Dynamic: id=id, />
                    }
                }
                Widget::Fixed(value) => {
                    html! {
                        <widgets::Fixed: value=value, />
                    }
                }
                Widget::Button(id) => {
                    html! {
                        <widgets::Button: />
                    }
                }
            },
            Layout::Row(ref layouts) => {
                html! {
                    <div class="rows",>
                        { for layouts.iter().map(|lyo| self.row(lyo)) }
                    </>
                }
            }
            Layout::Column(ref layouts) => {
                html! {
                    <div class="columns",>
                        { for layouts.iter().map(|lyo| self.column(lyo)) }
                    </>
                }
            }
        }
    }
}

impl Model {
    fn column(&self, layout: &Layout) -> View<Self> {
        html! {
            <div class="column",>
                <LayoutWidget: layout=Some(layout.clone()), />
            </div>
        }
    }

    fn row(&self, layout: &Layout) -> View<Self> {
        html! {
            <div class="row",>
                <LayoutWidget: layout=Some(layout.clone()), />
            </div>
        }
    }
}
