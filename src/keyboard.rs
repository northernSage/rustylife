use pixel_canvas::input::glutin::event::{ElementState, VirtualKeyCode};
use pixel_canvas::input::{Event, WindowEvent};
use pixel_canvas::canvas::CanvasInfo;

pub struct KeyboardState {
    //KeyboardInput { scancode: 0, state: Released, virtual_keycode: Some(A), modifiers: (empty) }
    pub scancode: u32,
    pub state: ElementState,
    pub virtual_key_code: VirtualKeyCode,
}

impl KeyboardState {
    /// Create a KeyboardState. For use with the `state` method.
    pub fn new() -> Self {
        Self {
            scancode: 0,
            state: ElementState::Pressed,
            virtual_key_code: VirtualKeyCode::Key0
        }
    }

    /// Handle input for the keyboard. For use with the `input` method.
    pub fn handle_input(info: &CanvasInfo, keyboard: &mut KeyboardState, event: &Event<()>) -> bool {
        match event {
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. }, ..
            } => {
                println!("{:?}", input);
                keyboard.scancode = input.scancode;
                keyboard.state = input.state;
                match input.virtual_keycode {
                    Some(code) => keyboard.virtual_key_code = code,
                    _ => (),
                }
                true
            }
            _ => false,
        }
    }
}