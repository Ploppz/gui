use crate::*;
use slog::{o, Discard, Logger};
use std::collections::HashMap;
use winit::event::{ElementState, ModifiersState, MouseButton};
use winput::{Input, MouseInput};

pub struct Expected {
    size: (f32, f32),
    pos: (f32, f32),
}

pub struct TestFixture {
    pub gui: Gui<NoDrawer>,
    pub input: Input,
    pub expected: HashMap<Id, Expected>,
}
impl TestFixture {
    const PADDING: f32 = 5.0;
    /// A configuration which is used in all tests
    pub fn fixture() -> Self {
        let mut gui = Gui::new(NoDrawer);
        gui.root = gui
            .root
            .padding(Self::PADDING, Self::PADDING, Self::PADDING, Self::PADDING);

        let mut expected = HashMap::new();
        let mut expected_x = Self::PADDING;
        for i in 0..10 {
            let id = if i < 5 {
                let id: usize = unimplemented!();
                gui.insert_widget_in_root(Button::new(String::new()).wrap(id));
                id
            } else {
                let id = unimplemented!();
                gui.insert_widget_in_root(ToggleButton::new(String::new()).wrap(id));
                id
            };
            // Set text field size (simulates rendering)
            gui.get_widget_mut(id)
                .unwrap()
                .children
                .values_mut()
                .next()
                .unwrap()
                .size = (50.0, 50.0);

            let expected_size = (
                // some extra padding of buttons
                50.0 + 6.0 * 2.0,
                50.0 + 4.0 * 2.0,
            );

            expected.insert(
                id.clone(),
                Expected {
                    size: expected_size,
                    pos: (expected_x, Self::PADDING),
                },
            );
            expected_x += expected_size.0
        }

        Self {
            gui,
            input: Input::default(),
            expected,
        }
    }
    /// Calls update on gui
    pub fn update(&mut self) -> (Vec<(Id, WidgetEvent)>, Capture) {
        let log = Logger::root(Discard, o!());
        let (events, capture) = self.gui.update(&self.input, log, &mut ());
        println!("[TestFixture][update] events = [");
        for event in events.iter() {
            let w = self.gui.get_widget(event.0).unwrap();
            print!("\t{:?}", event.1);
            match event.1 {
                WidgetEvent::ChangePos => print!("\tpos={:?}", w.pos),
                WidgetEvent::ChangeSize => print!("\tsize={:?}", w.size),
                _ => (),
            }
            println!("\tid={}", event.0);
        }
        println!("]\n");
        (events, capture)
    }
    /// Click (press and release) a widget. Returns (events, capture) after pressing and after
    /// releasing
    pub fn click_widget(
        &mut self,
        id: Id,
    ) -> (
        (Vec<(Id, WidgetEvent)>, Capture),
        (Vec<(Id, WidgetEvent)>, Capture),
    ) {
        let pos = self.expected[&id].pos;
        let size = self.expected[&id].size;
        let mouse_pos = (pos.0 + size.0 / 2.0, pos.1 + size.1 / 2.0);

        self.input.register_mouse_position(mouse_pos.0, mouse_pos.1);
        press_left_mouse(&mut self.input);
        println!("[TestFixture] Press mouse {:?}", mouse_pos);
        let result1 = self.update();
        release_left_mouse(&mut self.input);
        println!("[TestFixture] Release mouse down at {:?}", mouse_pos);
        let result2 = self.update();
        (result1, result2)
    }

    /*
    pub fn validate(&self) {
        self.update();
        for e in self.expected {
            // TODO assert expected is correct
        }
    }
    */
}
#[test]
fn test_fixture_expectation() {
    let mut fix = TestFixture::fixture();
    fix.update();
    fix.update();
    for (id, expected) in fix.expected.iter() {
        let w = fix.gui.get_widget(*id).unwrap();
        println!(
            "[{}]: pos: {:?} vs. {:?}, size: {:?} vs {:?}",
            id, w.pos, expected.pos, w.size, expected.size
        );
    }
    for (id, expected) in fix.expected.iter() {
        let real = fix.gui.get_widget(*id).unwrap();
        assert_eq!(expected.pos, real.pos);
        assert_eq!(expected.size, real.size);
    }
}

pub fn press_left_mouse(s: &mut Input) {
    s.register_mouse_input(
        MouseInput {
            state: ElementState::Pressed,
            modifiers: ModifiersState::default(),
        },
        MouseButton::Left,
    );
}
pub fn release_left_mouse(s: &mut Input) {
    s.register_mouse_input(
        MouseInput {
            state: ElementState::Released,
            modifiers: ModifiersState::default(),
        },
        MouseButton::Left,
    );
}
pub fn mouse_pressed() -> MouseInput {
    MouseInput {
        state: ElementState::Pressed,
        modifiers: ModifiersState::default(),
    }
}

#[macro_export]
macro_rules! assert_events {
    ($events:expr, $expected:expr) => {
        let mut events = $events.clone();
        let expected = $expected;
        let events_freeze = $events.clone();
        for expected_event in expected.iter() {
            if let Some(idx) = events.iter().enumerate().find_map(|(i, (_, e))| {
                if *e == *expected_event {
                    Some(i)
                } else {
                    None
                }
            }) {
                events.remove(idx);
            } else {
                panic!(
                    "\nAssertion failed: Event\n{:#?}\n\nnot in\n{:#?}",
                    expected_event, events_freeze
                );
            }
        }
    };
}

#[test]
fn print_fixture_widget_tree() {
    // Not really a test - but the tree printing isn't captured by test framework and thus best to
    // just once and for all show it
    use std::io::Write;
    writeln!(&mut std::io::stdout(), "TestFixture widget tree:").unwrap();
    print_widget_tree(&TestFixture::fixture().gui.root);
}
use ptree::{output::print_tree, TreeBuilder};
pub fn print_widget_tree(w: &Widget) {
    let mut tree = TreeBuilder::new(w.get_id().to_string());
    fn recurse(tree: &mut TreeBuilder, w: &Widget) {
        for child in w.children.values() {
            tree.begin_child(child.get_id().to_string());
            recurse(tree, &child);
            tree.end_child();
        }
    }
    recurse(&mut tree, w);
    let tree = tree.build();

    print_tree(&tree).unwrap();
}
