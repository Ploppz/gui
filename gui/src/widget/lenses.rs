use super::*;

// TODO(PosLens): should be possible to read the value indeed but not set it
// To set a value one should go through `config`!
// Perhaps `get_mut` somehow has to do that? Idk how.
#[derive(Clone)]
pub struct PosLens;
impl Lens for PosLens {
    type Source = Widget;
    type Target = Vec2;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
        &source.pos
    }
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
        &mut source.pos
    }
}
impl LeafLens for PosLens {
    fn target(&self) -> String {
        "Widget::pos".into()
    }
}

#[derive(Clone)]
pub struct SizeLens;
impl Lens for SizeLens {
    type Source = Widget;
    type Target = Vec2;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
        &source.size
    }
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
        &mut source.size
    }
}
impl LeafLens for SizeLens {
    fn target(&self) -> String {
        "Widget::size".into()
    }
}
#[derive(Clone)]
pub struct IdLens;
impl Lens for IdLens {
    type Source = Widget;
    type Target = Id;
    fn get<'a>(&self, source: &'a Widget) -> &'a Self::Target {
        &source.id
    }
    fn get_mut<'a>(&self, source: &'a mut Widget) -> &'a mut Self::Target {
        &mut source.id
    }
}
impl LeafLens for IdLens {
    fn target(&self) -> String {
        "Widget::id".into()
    }
}

#[derive(Clone)]
pub struct FirstChildLens;
impl Lens for FirstChildLens {
    type Source = Widget;
    type Target = Widget;
    fn get<'a>(&self, w: &'a Widget) -> &'a Widget {
        &w.children().values().next().unwrap()
    }
    fn get_mut<'a>(&self, w: &'a mut Widget) -> &'a mut Widget {
        w.children_mut().next().unwrap()
    }
}

#[derive(Clone)]
pub struct ChildLens {
    pub id: usize,
}
impl Lens for ChildLens {
    type Source = Widget;
    type Target = Widget;
    fn get<'a>(&self, w: &'a Widget) -> &'a Widget {
        &w.children[&self.id]
    }
    fn get_mut<'a>(&self, w: &'a mut Widget) -> &'a mut Widget {
        &mut w.children[&self.id]
    }
}

#[allow(non_upper_case_globals)]
impl Widget {
    pub const size: SizeLens = SizeLens;
    pub const pos: PosLens = PosLens;
    pub const first_child: FirstChildLens = FirstChildLens;
    pub const id: IdLens = IdLens;
    pub fn child_lens(id: usize) -> ChildLens {
        ChildLens { id }
    }
}
