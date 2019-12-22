use crate::gui::*;
use crate::*;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

/// note: not used directly - used in conjunction with a LensDriver
pub trait FieldLens {
    type Target;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target;
    fn put(&self, value: Self::Target) -> Box<dyn FnOnce(&mut Widget)>;
}

/// Support operations on widgets. Implementors will store Id or &mut Widget internally
pub trait LensDriver<L: FieldLens> {
    // fn queue_op<F: FnOnce(&mut Widget)>(&self, op: F);
    /// Gets field from widget
    fn get(&self, lens: L) -> L::Target;
    /// Mutate field of widget
    fn put(&self, lens: L, value: L::Target);
}

/// used for looking up widgets globally
pub struct WidgetLens<'a, D, L, I> {
    id: I,
    gui: &'a Gui<D>,
    _marker: PhantomData<L>,
}
impl<'a, D, L, I> WidgetLens<'a, D, L, I> {
    pub fn new(gui: &'a Gui<D>, id: I) -> Self {
        Self {
            id,
            gui,
            _marker: PhantomData,
        }
    }
}

impl<'a, D: GuiDrawer, L: FieldLens, I: AsId<D>> LensDriver<L> for WidgetLens<'a, D, L, I> {
    fn get(&self, lens: L) -> L::Target {
        lens.get(self.gui.get(self.id.clone()))
    }
    fn put(&self, lens: L, value: L::Target) {
        self.gui
            .op_service
            .borrow_mut()
            .push(self.id.resolve(&self.gui).unwrap(), lens.put(value))
    }
}

/// used for immediate children
/// only available internally (?)
pub struct ChildLens<'a, L> {
    child: &'a Widget,
    op_service: Rc<RefCell<OperationCell>>,
    _marker: PhantomData<L>,
}
impl<'a, L> ChildLens<'a, L> {
    pub fn new(child: &'a Widget, op_service: Rc<RefCell<OperationCell>>) -> Self {
        Self {
            child,
            op_service,
            _marker: PhantomData,
        }
    }
}

impl<'a, L: FieldLens> LensDriver<L> for ChildLens<'a, L> {
    fn get(&self, lens: L) -> L::Target {
        lens.get(&self.child)
    }
    fn put(&self, lens: L, value: L::Target) {
        self.op_service
            .borrow_mut()
            .push(self.child.get_id(), lens.put(value))
    }
}
