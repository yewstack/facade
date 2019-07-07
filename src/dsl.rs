#![allow(non_snake_case)]

use protocol::{self as p, Id, Layout, Scene, Value, Widget};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: Layout) -> Scene {
    Scene::FullScreen(value)
}

pub fn Spinner() -> Scene {
    Scene::Spinner
}

pub struct Dashboard<T>
where
    T: Into<Value>,
{
    pub title: T,
}

impl<T> Into<Scene> for Dashboard<T>
where
    T: Into<Value>,
{
    fn into(self) -> Scene {
        let dashboard = p::dashboard::Dashboard {
            title: self.title.into(),
            pages: vec![],
        };
        Scene::Dashboard(dashboard)
    }
}

/*
pub struct Dashboard<T, B, F>
where
    T: Into<Value>,
    B: Into<Layout>,
    F: Into<p::Footer>,
{
    pub title: T,
    pub body: B,
    pub footer: F,
}

impl<T, B, F> Into<Scene> for Dashboard<T, B, F>
where
    T: Into<Value>,
    B: Into<Layout>,
    F: Into<p::Footer>,
{
    fn into(self) -> Scene {
        Scene::Dashboard {
            title: self.title.into(),
            body: self.body.into(),
            footer: self.footer.into(),
        }
    }
}

pub struct Footer<C, M>
where
    C: Into<Value>,
    M: Into<p::Menu>,
{
    pub copyright: C,
    pub menu: M,
}

impl<C, M> Into<p::Footer> for Footer<C, M>
where
    C: Into<Value>,
    M: Into<p::Menu>,
{
    fn into(self) -> p::Footer {
        p::Footer {
            copyright: self.copyright.into(),
            menu: self.menu.into(),
        }
    }
}
*/

pub fn Menu(value: impl IntoIterator<Item = p::MenuItem>) -> p::Menu {
    p::Menu {
        items: value.into_iter().collect(),
    }
}

pub fn Item(value: impl Into<Value>) -> p::MenuItem {
    p::MenuItem {
        caption: value.into(),
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
