use crate::widgets::{Reqs, View, Widget, WidgetModel};
use protocol::Value;
use yew::html;

pub type Spinner = WidgetModel<Model>;

#[derive(Default)]
pub struct Model { }

impl Widget for Model {
    type Properties = ();

    fn main_view(&self) -> View<Self> {
        html! {
            <div class="spinner",>
                <img src="./spinner.svg", width=200, />
            </div>
        }
    }
}
