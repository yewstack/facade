use crate::live::{Requirement, ResponseEvt};
use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::{Reaction, Scene};
use yew::{html, Properties, ShouldRender};

pub type SceneWidget = WidgetModel<Model>;

pub struct Model {
    scene: Scene,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub scene: Option<Scene>,
}

impl Widget for Model {
    type Message = ();
    type Properties = Props;

    fn produce(_: &Self::Properties) -> Self {
        Self {
            scene: Scene::Spinner,
        }
    }

    fn recompose(&mut self, _: &Self::Properties) -> Reqs {
        Some(vec![Requirement::SceneChange].into_iter().collect())
    }

    fn handle_incoming(&mut self, event: ResponseEvt) -> ShouldRender {
        if let ResponseEvt::Reaction(Reaction::Scene(scene)) = event {
            log::info!("Changing scene: {:?}", scene);
            self.scene = scene;
            true
        } else {
            false
        }
    }

    fn main_view(&self) -> View<Self> {
        match self.scene {
            Scene::Spinner => {
                html! {
                    <widgets::Spinner: />
                }
            }
            Scene::FullScreen(ref layout) => {
                html! {
                    <div class="scene-fullscreen",>
                        <widgets::Layout: layout=layout.clone(), />
                    </div>
                }
            }
            Scene::Dashboard(ref dashboard) => {
                html! {
                    <widgets::Dashboard: dashboard=dashboard.clone(), />
                }
            }
        }
    }
}
