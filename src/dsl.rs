#![allow(non_snake_case)]

use protocol::{Id, Layout, Scene, Value, Widget};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: Layout) -> Scene {
    Scene::FullScreen(value)
}

pub fn Spinner() -> Scene {
    Scene::Spinner
}

pub struct Dashboard<T, B>
where
    T: Into<Value>,
    B: Into<Layout>,
{
    pub title: T,
    pub body: B,
}

impl<T, B> Into<Scene> for Dashboard<T, B>
where
    T: Into<Value>,
    B: Into<Layout>,
{
    fn into(self) -> Scene {
        Scene::Dashboard {
            title: self.title.into(),
            body: self.body.into(),
        }
    }
}

// ╔═╗┌─┐┌┐┌┌┬┐┌─┐┬┌┐┌┌─┐┬─┐┌─┐
// ║  │ ││││ │ ├─┤││││├┤ ├┬┘└─┐
// ╚═╝└─┘┘└┘ ┴ ┴ ┴┴┘└┘└─┘┴└─└─┘

// ╦  ┌─┐┬ ┬┌─┐┬ ┬┌┬┐
// ║  ├─┤└┬┘│ ││ │ │
// ╩═╝┴ ┴ ┴ └─┘└─┘ ┴

pub fn Row(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Row(layouts)
}

pub fn Column(value: impl IntoIterator<Item = Layout>) -> Layout {
    let layouts = value.into_iter().collect();
    Layout::Column(layouts)
}

// ╦ ╦┬┌┬┐┌─┐┌─┐┌┬┐┌─┐
// ║║║│ │││ ┬├┤  │ └─┐
// ╚╩╝┴─┴┘└─┘└─┘ ┴ └─┘

pub fn Dynamic(value: impl AsRef<str>) -> Layout {
    let id = Id::from(value);
    Layout::Widget(Widget::Dynamic(id))
}

pub fn Fixed(value: impl Into<Value>) -> Layout {
    Layout::Widget(Widget::Fixed(value.into()))
}
