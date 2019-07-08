//! This module defines aliases to widget types.

mod button;
pub use button::Button;

mod container;
pub use container::ContainerWidget as Container;

mod dashboard;
pub use dashboard::Dashboard;

mod dynamic;
pub use dynamic::Dynamic;

mod fixed;
pub use fixed::Fixed;

mod layout;
pub use layout::LayoutWidget as Layout;

mod page;
pub use page::Page;

mod scene;
pub use scene::SceneWidget as Scene;

mod spinner;
pub use spinner::Spinner;

mod welcome;
pub use welcome::Welcome;

mod widget;
pub use widget::{Reqs, View, Widget, WidgetModel};
