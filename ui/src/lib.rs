mod live;

use live::{Live, ResponseEvt};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, Renderable, ShouldRender};

pub struct Model {
    connection: Box<dyn Bridge<Live>>,
    counter: u64,
}

pub enum Msg {
    Click,
    Event(ResponseEvt),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        let callback = link.send_back(Msg::Event);
        let connection = Live::bridge(callback);
        Model {
            connection,
            counter: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.counter += 1;
            }
            Msg::Event(_) => {
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
