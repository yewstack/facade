#![allow(non_snake_case)]

use protocol::{self as frame, Id, Layout, Value, Widget};

// ╔═╗┌─┐┌─┐┌┐┌┌─┐┌─┐
// ╚═╗│  ├┤ │││├┤ └─┐
// ╚═╝└─┘└─┘┘└┘└─┘└─┘

pub fn FullScreen(value: Layout) -> frame::Scene {
    frame::Scene::FullScreen(value)
}

pub fn Spinner() -> frame::Scene {
    frame::Scene::Spinner
}

pub fn Dashboard(
    title: impl Into<Value>,
    pages: impl IntoIterator<Item = frame::dashboard::Page>,
) -> frame::Scene {
    let dashboard = frame::dashboard::Dashboard {
        title: title.into(),
        pages: pages.into_iter().collect(),
    };
    frame::Scene::Dashboard(dashboard)
}

pub fn Page(
    title: impl Into<Value>,
    subtitle: impl Into<Value>,
    body: impl Into<Layout>,
) -> frame::dashboard::Page {
    frame::dashboard::Page {
        title: title.into(),
        subtitle: subtitle.into(),
        body: body.into(),
    }
}

/*
pub struct Dashboard<T>
where
    T: Into<Value>,
{
    pub title: T,
    pub pages: Vec<frame::dashboard::Page>,
}

impl<T> Into<Scene> for Dashboard<T>
where
    T: Into<Value>,
{
    fn into(self) -> Scene {
        let dashboard = frame::dashboard::Dashboard {
            title: self.title.into(),
            pages: self.pages.into_iter().map(,
        };
        Scene::Dashboard(dashboard)
    }
}

pub struct Page<T>
where
    T: Into<Value>,
{
    pub title: T,
}

impl<T> Into<frame::dashboard::Page> for Page<T>
where
    T: Into<Value>,
{
    fn into(self) -> frame::dashboard::Page {
        frame::dashboard::Page {
            title: self.title.into(),
            subtitle: "".into(),
            body: Layout::Blank,
        }
    }
}
*/


/*
pub struct Dashboard<T, B, F>
where
    T: Into<Value>,
    B: Into<Layout>,
    F: Into<frame::Footer>,
{
    pub title: T,
    pub body: B,
    pub footer: F,
}

impl<T, B, F> Into<Scene> for Dashboard<T, B, F>
where
    T: Into<Value>,
    B: Into<Layout>,
    F: Into<frame::Footer>,
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
    M: Into<frame::Menu>,
{
    pub copyright: C,
    pub menu: M,
}

impl<C, M> Into<frame::Footer> for Footer<C, M>
where
    C: Into<Value>,
    M: Into<frame::Menu>,
{
    fn into(self) -> frame::Footer {
        frame::Footer {
            copyright: self.copyright.into(),
            menu: self.menu.into(),
        }
    }
}
*/

pub fn Menu(value: impl IntoIterator<Item = frame::MenuItem>) -> frame::Menu {
    frame::Menu {
        items: value.into_iter().collect(),
    }
}

pub fn Item(value: impl Into<Value>) -> frame::MenuItem {
    frame::MenuItem {
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
