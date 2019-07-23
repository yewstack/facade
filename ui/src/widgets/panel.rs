use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use yew::{html, Properties};

pub type PanelWidget = WidgetModel<Model>;

pub struct Model {
    panel: protocol::Panel,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    #[props(required)]
    pub panel: protocol::Panel,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(props: &Self::Properties) -> Self {
        Self { panel: props.panel.clone() }
    }

    fn recompose(&mut self, props: &Self::Properties) -> Reqs {
        self.panel = props.panel.to_owned();
        None
    }

    fn main_view(&self) -> View<Self> {
        if let Some(ref title) = self.panel.title {
            html! {
                <div class="panel",>
                    <div class="panel-header",>
                        <p class="panel-header-title",>{ title }</p>
                    </div>
                    <div class="panel-content",>
                        <widgets::Layout: layout=self.panel.body.clone(), />
                    </div>
                </div>
            }
        } else {
            html! {
                <div class="panel",>
                    <div class="panel-content",>
                        <widgets::Layout: layout=self.panel.body.clone(), />
                    </div>
                </div>
            }
        }
    }
}
