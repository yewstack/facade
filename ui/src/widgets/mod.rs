//! This module defines aliases to widget types.

/*
mod bind;
pub use bind::BindWidget as Bind;

mod blank;
pub use blank::BlankWidget as Blank;

mod button;
pub use button::ButtonWidget as Button;

mod control;
pub use control::ControlWidget as Control;

mod dashboard;
pub use dashboard::DashboardWidget as Dashboard;

mod dynamic;
pub use dynamic::DynamicWidget as Dynamic;

mod fixed;
pub use fixed::FixedWidget as Fixed;

mod page;
pub use page::PageWidget as Page;

mod panel;
pub use panel::PanelWidget as Panel;

mod welcome;
pub use welcome::WelcomeWidget as Welcome;
*/

mod app;
pub use app::AppWidget as App;

mod container;
pub use container::ContainerWidget as Container;

mod icon;
pub use icon::Icon;

mod layout;
pub use layout::LayoutWidget as Layout;

mod list;
pub use list::ListWidget as List;

mod scene;
pub use scene::SceneWidget as Scene;

mod spinner;
pub use spinner::SpinnerWidget as Spinner;

mod widget;
pub use widget::{Reqs, View, Widget, WidgetModel};
