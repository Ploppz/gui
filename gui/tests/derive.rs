use gui::{lens::*, *};

/*
#[derive(LensInternal, Debug)]
struct Foo {
    #[lens]
    a: i32,
    #[lens]
    b: String,
}
impl Interactive for Foo {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
*/

#[derive(Lens, Debug)]
pub struct Bar<T> {
    #[lens]
    a: i32,
    #[lens]
    b: String,
    t: T,
}
impl<T> Bar<T> {
    pub fn new(t: T) -> Self {
        Bar {
            a: 0,
            b: "hey".into(),
            t,
        }
    }
}
impl<T: Send + Sync + std::fmt::Debug + 'static> Interactive for Bar<T> {
    fn captures(&self) -> Capture {
        Capture {
            mouse: false,
            keyboard: false,
        }
    }
}
type DefaultBar = Bar<i32>;

#[test]
fn test_lenses() {
    let mut gui = Gui::new(NoDrawer, &mut ());
    gui.insert_in_root_with_alias(Bar::<i32>::new(0), "abc".to_string());
    gui.access("abc").chain(DefaultBar::a).put(2);
    assert_eq!(2, *gui.access("abc").chain(DefaultBar::a).get())
}
