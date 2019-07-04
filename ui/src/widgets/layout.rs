use crate::live::{Requirement, ResponseEvt};
use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Reaction;
use yew::{html, ShouldRender};

pub type Layout = WidgetModel<Model>;

pub struct Model {
    layout: protocol::Layout,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            layout: protocol::Layout::Blank,
        }
    }
}

impl Widget for Model {
    fn requirements(&self) -> Reqs {
        vec![Requirement::LayoutChange].into_iter().collect()
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
        use protocol::{Layout, Widget};
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
                        <widgets::Dynamic: />
                    }
                }
                Widget::Fixed(value) => {
                    html! {
                        <widgets::Fixed: />
                    }
                }
                Widget::Button(id) => {
                    html! {
                        <widgets::Button: />
                    }
                }
            },
            Layout::Row(_) => {
                html! {
                    <p>{ "Row" }</p>
                }
            }
            Layout::Column(_) => {
                html! {
                    <p>{ "Column" }</p>
                }
            }
        }
    }
}
