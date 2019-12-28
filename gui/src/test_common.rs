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
    pub expected: HashMap<String, Expected>,
}
impl TestFixture {
    const PADDING: f32 = 5.0;
    /// A configuration which is used in all tests
    pub fn fixture() -> Self {
        let mut gui = Gui::new(NoDrawer);
        gui.root.config =
            gui.root
                .config
                .padding(Self::PADDING, Self::PADDING, Self::PADDING, Self::PADDING);

        let mut expected = HashMap::new();
        let mut expected_x = Self::PADDING;
        for i in 0..10 {
            let id = if i < 5 {
                let id: String = format!("Button {}", i);
                gui.insert_in_root_with_alias(Button::new(), id.clone());
                id
            } else {
                let id: String = format!("ToggleButton {}", i - 5);
                gui.insert_in_root_with_alias(ToggleButton::new(), id.clone());
                id
            };

            // Set text field size (simulates rendering)
            gui.get_mut(&id)
                .children
                .values_mut()
                .next()
                .unwrap()
                .config
                .set_size(50.0, 50.0);

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
    pub fn update(&mut self) -> (Vec<Event>, Capture) {
        let log = Logger::root(Discard, o!());
        let (events, capture) = self.gui.update(&self.input, log, &mut ());
        println!("[TestFixture][update] events = [");
        for event in events.iter() {
            let w = self.gui.get(event.id);
            print!("\t{:?}", event.kind);
            if let EventKind::Change { ref field } = event.kind {
                if field.is_pos() {
                    print!("\tpos={:?}", w.pos);
                } else if field.is_size() {
                    print!("\tsize={:?}", w.size);
                }
            }
            println!("\tid={}", event.id);
        }
        println!("]\n");
        (events, capture)
    }
    /// Click (press and release) a widget. Returns (events, capture) after pressing and after
    /// releasing
    pub fn click_widget(&mut self, id: &str) -> ((Vec<Event>, Capture), (Vec<Event>, Capture)) {
        let pos = self.expected[id].pos;
        let size = self.expected[id].size;
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
        let w = fix.gui.get(id.as_str());
        println!(
            "[{}]: pos: {:?} vs. {:?}, size: {:?} vs {:?}",
            id, w.pos, expected.pos, w.size, expected.size
        );
        println!(
            "[{}]: size hints: {:?}",
            id,
            (w.config.size_hint_x, w.config.size_hint_y)
        );
    }
    for (id, expected) in fix.expected.iter() {
        let real = fix.gui.get(id.as_str());
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
            if let Some(idx) = events.iter().enumerate().find_map(|(i, event)| {
                if event.kind == *expected_event {
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
    print_widget_tree(TestFixture::fixture().gui);
}
use ptree::{output::print_tree, TreeBuilder};
pub fn print_widget_tree<D: GuiDrawer>(gui: Gui<D>) {
    let aliases = gui
        .aliases
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<IndexMap<usize, String>>();
    let mut tree = TreeBuilder::new(gui.root.get_id().to_string());
    fn recurse<E: GuiDrawer>(
        tree: &mut TreeBuilder,
        w: &Widget,
        gui: &Gui<E>,
        aliases: &IndexMap<usize, String>,
    ) {
        for child in w.children.values() {
            let name = if let Some(alias) = aliases.get(&child.get_id()) {
                format!("{} \"{}\"", child.get_id(), alias)
            } else {
                child.get_id().to_string()
            };
            tree.begin_child(name);
            recurse(tree, &child, gui, aliases);
            tree.end_child();
        }
    }
    recurse(&mut tree, &gui.root, &gui, &aliases);
    let tree = tree.build();

    print_tree(&tree).unwrap();
}
