use crate::gui::*;
use crate::*;

/// note: not used directly - used in conjunction with a LensDriver
pub trait FieldLens: 'static {
    type Target: PartialEq;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target;
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target;
    // TODO FUTURE: `fn cmp_targets` to support Targets that are not PartialEq. (-> always emit
    // Change event)
}

/// Support operations on widgets. Implementors will store Id or &mut Widget internally
pub trait LensDriver {
    // fn queue_op<F: FnOnce(&mut Widget)>(&self, op: F);
    /// Gets field from widget
    fn get<L: FieldLens>(&self, lens: L) -> &L::Target;
    /// Mutate field of widget
    fn put<L: FieldLens>(&mut self, lens: L, value: L::Target) -> &mut Self;
}

/// used for looking up widgets globally
pub struct WidgetLens<'a, D, I> {
    id: I,
    gui: &'a mut Gui<D>,
}
impl<'a, D, I> WidgetLens<'a, D, I> {
    pub fn new(gui: &'a mut Gui<D>, id: I) -> Self {
        Self { id, gui }
    }
}

impl<'a, D: GuiDrawer, I: AsId<D>> LensDriver for WidgetLens<'a, D, I> {
    fn get<L: FieldLens>(&self, lens: L) -> &L::Target {
        lens.get(self.gui.get(self.id.clone()))
    }
    fn put<L: FieldLens>(&mut self, lens: L, value: L::Target) -> &mut Self {
        let target = lens.get_mut(self.gui.get_mut(self.id.clone()));
        let equal = *target == value;
        if !equal {
            *target = value;
            let id = self.id.resolve(&self.gui).unwrap();
            self.gui
                .internal
                .borrow_mut()
                .push_event(Event::change(id, lens))
        }
        self
    }
}

/// Used only internally in `Interactive`
pub(crate) struct InternalLens<'a, 'b> {
    widget: &'a mut Widget,
    events: &'b mut Vec<Event>,
}
impl<'a, 'b> InternalLens<'a, 'b> {
    pub fn new(widget: &'a mut Widget, events: &'b mut Vec<Event>) -> Self {
        Self { widget, events }
    }
}

impl<'a, 'b> LensDriver for InternalLens<'a, 'b> {
    fn get<L: FieldLens>(&self, lens: L) -> &L::Target {
        lens.get(&self.widget)
    }
    fn put<L: FieldLens>(&mut self, lens: L, value: L::Target) -> &mut Self {
        let target = lens.get_mut(self.widget);
        let equal = *target == value;
        if !equal {
            *target = value;
            self.events.push(Event::change(self.widget.get_id(), lens));
        }
        self
    }
}
