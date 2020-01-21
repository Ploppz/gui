//! Collection of useful widgets such as button, container, dropdown, and text fields.
//!
//! All widgets have a type parameter called `Style`, because the style may differ from renderer to
//! renderer ([crate::gui::GuiDrawer]). All it is, is data that and that the user can set to configure the
//! appearance of widgets.
//! `gui` does not assume anything about how appearance is represented.
//! Note that because some widgets depend on other widgets (for example, [Button] depends
//! on [TextField]), these widgets must also be parameterized by the `Style` of those dependees.
//!
mod button;
mod container;
mod dropdown;
mod text;

pub use button::*;
pub use container::*;
pub use dropdown::*;
pub use text::*;

/// Really just a shortcut used internally (has to be `pub`). Ignore.
pub trait StyleBound: Default + Send + Sync + Clone + std::fmt::Debug + 'static {}

impl<T> StyleBound for T where T: Default + Send + Sync + Clone + std::fmt::Debug + 'static {}

impl ContainerStyle for () {}
impl TextFieldStyle for () {}
impl ButtonStyle for () {
    type TextField = ();
}
impl DropdownButtonStyle for () {
    type TextField = ();
    type Button = ();
}
