use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::Container;
use yew::html;

pub type ContainerWidget = WidgetModel<Model>;

pub struct Model {
    container: Container,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            container: Container::Blank,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub container: Option<Container>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        if let Some(ref container) = props.container {
            self.container = container.clone();
        }
        None
    }

    fn main_view(&self) -> View<Self> {
        match self.container {
            Container::Blank => {
                html! {
                    <p>{ "Blank" }</p>
                }
            }
            Container::Tabs(_) => {
                html! {
                    <p>{ "Tabs" }</p>
                }
            }
            Container::Panel(ref panel) => {
                html! {
                    <div class="container",>
                        <widgets::Panel: panel=Some(panel.clone()), />
                    </div>
                }
            }
        }
    }
}
