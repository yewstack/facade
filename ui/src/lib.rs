use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    counter: u64,
}

pub enum Msg {
    Click,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.counter += 1;
            }
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <button onclick=|_| Msg::Click,>{ "Click" }</button>
                <p>{ format!("Counter: {}", self.counter) }</p>
            </div>
        }
    }
}
