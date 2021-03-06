use crate::default;
use crate::*;
use slog::{o, Discard, Logger};
use std::collections::HashMap;
use winit::event::{ElementState, ModifiersState, MouseButton};
use winput::{Input, MouseInput};

pub type TextField = default::TextField<()>;
pub type Button = default::Button<()>;
pub type ToggleButton = default::ToggleButton<()>;
pub type Select = default::Select<()>;

#[derive(Deref, DerefMut)]
pub struct TestGui {
    #[deref_target]
    pub gui: Gui<NoDrawer>,
    pub input: Input,
    pub log: Logger,
}
impl TestGui {
    pub fn new() -> Self {
        TestGui {
            log: Logger::root(Discard, o!()),
            gui: Gui::new(NoDrawer, &mut ()),
            input: Input::default(),
        }
    }
    fn update_internal(&mut self) -> (Vec<Event>, Capture) {
        let (events, capture) = self.gui.update(&self.input, self.log.clone(), &mut ());

        println!("[TestGui.update] events = [");
        for event in events.iter() {
            let w = self.gui.get(event.id);
            print!("\t{:?}", event.kind);
            if let EventKind::Change { ref field } = event.kind {
                if field.is_pos() {
                    print!("\tpos={}", w.pos);
                } else if field.is_size() {
                    print!("\tsize={}", w.size);
                }
            }
            println!("\tid={}", event.id);
        }
        println!("]\n");
        (events, capture)
    }
    /// Simulate a frame
    pub fn update(&mut self) -> (Vec<Event>, Capture) {
        self.input.prepare_for_next_frame();
        self.update_internal()
    }
    /// Simulate a frame in which user presses left mouse button down.
    pub fn press(&mut self, pos: Vec2) -> (Vec<Event>, Capture) {
        self.input.prepare_for_next_frame();
        self.input.register_mouse_position(pos.x, pos.y);
        press_left_mouse(&mut self.input);
        self.update_internal()
    }
    /// Simulate a frame in which user releases left mouse button down.
    pub fn release(&mut self) -> (Vec<Event>, Capture) {
        self.input.prepare_for_next_frame();
        release_left_mouse(&mut self.input);
        self.update_internal()
    }
}

pub struct Expected {
    size: Vec2,
    pos: Vec2,
}

pub struct TestFixture {
    pub gui: TestGui,
    pub expected: HashMap<String, Expected>,
}
impl TestFixture {
    const PADDING: f32 = 5.0;
    /// A configuration which is used in all tests
    pub fn fixture() -> Self {
        let mut test_gui = TestGui::new();
        let mut gui = &mut test_gui.gui;
        gui.root.config =
            gui.root
                .config
                .padding(Self::PADDING, Self::PADDING, Self::PADDING, Self::PADDING);

        let mut expected = HashMap::new();
        let mut expected_x = Self::PADDING;
        for i in 0..10 {
            let id = match i {
                0..=4 => {
                    let id: String = format!("Button {}", i);
                    gui.insert_in_root_with_alias(Button::new(), id.clone());
                    id
                }
                5..=9 => {
                    let id: String = format!("ToggleButton {}", i - 5);
                    gui.insert_in_root_with_alias(ToggleButton::new(), id.clone());
                    id
                }
                _ => unreachable!(),
            };

            gui.access(&id)
                .chain(Widget::first_child)
                .chain(TextField::text)
                .put("text".to_string());

            let text_size = NoDrawer.text_calc(0, &mut ()).text_size("text");

            // expected button size.
            // Per now, buttons have constant height, and padding in X axis
            let expected_size = Vec2::<f32>::new(
                text_size.x + 6.0 * 2.0, // 6.0 padding on two sides
                crate::default::DEFAULT_BUTTON_HEIGHT,
            );

            expected.insert(
                id.clone(),
                Expected {
                    size: expected_size,
                    pos: Vec2::new(expected_x, Self::PADDING),
                },
            );
            expected_x += expected_size.x
        }

        Self {
            gui: test_gui,
            expected,
        }
    }
    /// Calls update on gui
    pub fn update(&mut self) -> (Vec<Event>, Capture) {
        self.gui.update()
    }
    /// Click (press and release) a widget. Returns (events, capture) after pressing and after
    /// releasing
    pub fn click_widget(&mut self, id: &str) -> ((Vec<Event>, Capture), (Vec<Event>, Capture)) {
        let pos = self.expected[id].pos;
        let size = self.expected[id].size;
        let mouse_pos = pos + size / 2.0;

        (self.gui.press(mouse_pos), self.gui.release())
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
    print_widget_tree(&fix.gui, |w| {
        let alias =
            fix.gui
                .aliases
                .iter()
                .find_map(|(alias, id)| if *id == w.get_id() { Some(alias) } else { None });
        if let Some(alias) = alias {
            let expected = &fix.expected[alias];
            format!(
                "pos={} exp={} - size{} exp={}",
                w.pos, expected.pos, w.size, expected.size
            )
        } else {
            format!("pos={} - size={}", w.pos, w.size)
        }
    });
    /*
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
    */
    for (id, expected) in fix.expected.iter() {
        let real = fix.gui.get(id.as_str());
        assert_eq!(expected.pos, real.pos);
        assert_eq!(expected.size, real.size);
    }
}

pub fn press_left_mouse(s: &mut Input) {
    s.register_mouse_input(&ElementState::Pressed, &MouseButton::Left);
}

pub fn release_left_mouse(s: &mut Input) {
    s.register_mouse_input(&ElementState::Released, &MouseButton::Left);
}
pub fn new_frame(s: &mut Input) {
    s.prepare_for_next_frame();
}
pub fn mouse_pressed() -> MouseInput {
    MouseInput {
        state: ElementState::Pressed,
        modifiers: ModifiersState::default(),
    }
}

#[test]
fn test_testing() {
    // just validate some assumptions
    let mut input = Input::default();
    press_left_mouse(&mut input);
    assert!(input.is_mouse_button_toggled_down(winit::event::MouseButton::Left));
    new_frame(&mut input);
    assert!(!input.is_mouse_button_toggled_down(winit::event::MouseButton::Left));
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
    print_widget_tree(&TestFixture::fixture().gui, |w| {
        format!("      pos{:?} size{:?}", w.pos, w.size)
    });
}
use ptree::{output::print_tree, TreeBuilder};
pub fn print_widget_tree<D: GuiDrawer, F: Fn(&Widget) -> String>(gui: &Gui<D>, info: F) {
    let aliases = gui
        .aliases
        .iter()
        .map(|(k, v)| (*v, k.clone()))
        .collect::<IndexMap<usize, String>>();
    let mut tree = TreeBuilder::new(gui.root.get_id().to_string());
    fn recurse<E: GuiDrawer, F: Fn(&Widget) -> String>(
        tree: &mut TreeBuilder,
        w: &Widget,
        gui: &Gui<E>,
        aliases: &IndexMap<usize, String>,
        info: &F,
    ) {
        for child in w.children.values() {
            let name = if let Some(alias) = aliases.get(&child.get_id()) {
                format!("{} \"{}\"", child.get_id(), alias)
            } else {
                child.get_id().to_string()
            };
            let name = format!("{} {}", name, info(child));
            tree.begin_child(name);
            recurse(tree, &child, gui, aliases, info);
            tree.end_child();
        }
    }
    recurse(&mut tree, &gui.root, &gui, &aliases, &info);
    let tree = tree.build();

    print_tree(&tree).unwrap();
}
