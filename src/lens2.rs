use crate::gui::*;
use crate::*;
use std::{cell::RefCell, marker::PhantomData, rc::Rc};

pub trait LazyLens {
    type Source;
    type Target;
    type Cascade;
    /// Used for getting immutable values.
    fn with<V, F: FnOnce(&Self::Target) -> V>(&self, data: &Self::Source, f: F) -> V;
    // reserved for RootLens:
    // fn put<F: FnOnce(&mut Self::Target) + 'static>(&self, data: &mut Self::Source, f: F);
    /// Used to construct a closure to mutate
    fn cascade<F: FnOnce(&mut Self::Target)>(&self, f: F) -> Self::Cascade;
}
pub trait LazyLensExt: LazyLens {
    fn get(&self, data: &Self::Source) -> Self::Target
    where
        Self::Target: Clone,
    {
        self.with(data, |x| x.clone())
    }

    fn then<M, L: LazyLens>(self, other: L) -> Then<Self, L>
    where
        Self: LazyLens<Target = M> + Sized,
        L: LazyLens<Source = M>,
    {
        Then {
            left: self,
            right: other,
        }
    }
}
impl<T: LazyLens> LazyLensExt for T {}

//-------------
// Widget lens
//_____________

// Gui -> Widget
pub struct WidgetLens<D> {
    id: Id,
    // ops: Rc<RefCell<OperationService>>,
    marker: PhantomData<D>,
}
impl<D: GuiDrawer> WidgetLens<D> {
    pub fn then<M, L>(self, other: L) -> GuiThen<D, L>
    where
        L: LazyLens<Source = Widget>,
    {
        GuiThen {
            left: self,
            right: other,
        }
    }
}
impl<D: GuiDrawer> LazyLens for WidgetLens<D> {
    type Source = Gui<D>;
    type Target = Widget;
    type Cascade = impl FnOnce(&mut Gui<D>);
    // Idea: this can be used simply to `get` a value
    fn with<V, F: FnOnce(&Widget) -> V>(&self, data: &Gui<D>, f: F) -> V {
        f(
            unimplemented!(), // TODO: fetch widget
        )
    }

    // Idea: this is used to push `f` to self.ops
    // thus `f` is not called,
    // and its return value is ignored
    // PROBLEM: need to return the  V... impossible

    fn cascade<F: FnOnce(&mut Self::Target)>(&self, f: F) -> Self::Cascade {
        move |gui| gui.op_service.borrow_mut().push(self.id, f)
    }
}

//--------
// GuiThen
//________

pub struct GuiThen<D, L: LazyLens> {
    left: WidgetLens<D>,
    right: L,
}
impl<D, L> LazyLens for GuiThen<D, L>
where
    L: LazyLens<Source = Widget>,
    D: GuiDrawer,
{
    type Source = Gui<D>;
    type Target = L::Target;
    fn with<V, F: FnOnce(&Self::Target) -> V>(&self, data: &Self::Source, f: F) -> V {
        self.left.with(data, |b| self.right.with(b, f))
    }
    fn put<F: FnOnce(&mut Self::Target) + 'static>(&self, data: &mut Self::Source, f: F) {
        // store the closure in

        // TODO:
        // so.. left is Gui -> Widget,
        // right is Widget -> field
        // won't really work in a nice way? Target is not part of `L1`. The fn needs to be boxed.
    }
}
