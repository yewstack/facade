use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type PanelWidget = WidgetModel<Model>;

pub struct Model {
    panel: Option<protocol::Panel>,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub panel: Option<protocol::Panel>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self { panel: None }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.panel = props.panel.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(panel) = self.panel.as_ref() {
            if let Some(ref title) = panel.title {
                html! {
                    <div class="panel",>
                        <div class="panel-header",>
                            <p class="panel-header-title",>{ title }</p>
                        </div>
                        <div class="panel-content",>
                            <widgets::Layout: layout=panel.body.clone(), />
                        </div>
                    </div>
                }
            } else {
                html! {
                    <div class="panel",>
                        <div class="panel-content",>
                            <widgets::Layout: layout=panel.body.clone(), />
                        </div>
                    </div>
                }
            }
        } else {
            html! {
                <widgets::Spinner: />
            }
        }
    }
}
