use crate::live::{Requirement, ResponseEvt};
use crate::widgets::{self, Reqs, View, Widget, WidgetModel};
use protocol::{Reaction, Scene};
use yew::{html, ShouldRender};

pub type SceneWidget = WidgetModel<Model>;

pub struct Model {
    scene: Scene,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            scene: Scene::Spinner,
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Props {
    pub scene: Option<Scene>,
}

impl Widget for Model {
    type Properties = Props;

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
                // TODO Maybe move to a component?
                html! {
                    <div class="scene-spinner",>
                        <img src="./spinner.svg", width=200, />
                    </div>
                }
            }
            Scene::FullScreen(ref layout) => {
                html! {
                    <div class="scene-fullscreen",>
                        <widgets::Layout: layout=Some(layout.clone()), />
                    </div>
                }
            }
            Scene::Dashboard { ref title, ref body } => {
                html! {
                    <div class="scene-dashboard",>
                        <div class="header",>
                        </div>
                        <div class="body",>
                        </div>
                        <div class="footer",>
                        </div>
                    </div>
                }
            }
        }
    }
}
