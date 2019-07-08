//! This module defines aliases to widget types.

mod blank;
pub use blank::BlankWidget as Blank;

mod button;
pub use button::ButtonWidget as Button;

mod container;
pub use container::ContainerWidget as Container;

mod dashboard;
pub use dashboard::DashboardWidget as Dashboard;

mod dynamic;
pub use dynamic::DynamicWidget as Dynamic;

mod fixed;
pub use fixed::FixedWidget as Fixed;

mod layout;
pub use layout::LayoutWidget as Layout;

mod page;
pub use page::PageWidget as Page;

mod panel;
pub use panel::PanelWidget as Panel;

mod scene;
pub use scene::SceneWidget as Scene;

mod spinner;
pub use spinner::SpinnerWidget as Spinner;

mod welcome;
pub use welcome::WelcomeWidget as Welcome;

mod widget;
pub use widget::{Reqs, View, Widget, WidgetModel};
