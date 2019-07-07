use crate::widgets::{View, Widget, WidgetModel};
use yew::html;

pub type Blank = WidgetModel<Model>;

#[derive(Default)]
pub struct Model { }

impl Widget for Model {
    type Properties = ();

    fn main_view(&self) -> View<Self> {
        html! {
            <div class="blank",>
                <img src="./blank.svg", width=200, />
            </div>
        }
    }
}
