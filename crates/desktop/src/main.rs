use lean_editor_core::Document;
use std::env;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, KeyCode, ModifiersState, NamedKey, PhysicalKey},
    window::{Window, WindowId},
};

#[derive(Default)]
struct App {
    window: Option<Window>,
    document: Option<Document>,
    modifiers: ModifiersState,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("Lean Editor");
            let window = event_loop.create_window(window_attributes).unwrap();
            self.window = Some(window);
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::KeyS),
                        ..
                    },
                ..
            } => {
                if self.modifiers.control_key() || self.modifiers.super_key() {
                    if let Some(doc) = &mut self.document {
                        if let Err(e) = doc.save() {
                            eprintln!("Failed to save document: {}", e);
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::KeyZ),
                        ..
                    },
                ..
            } => {
                let cmd_or_ctrl = self.modifiers.control_key() || self.modifiers.super_key();
                let shift = self.modifiers.shift_key();

                if cmd_or_ctrl {
                    if let Some(doc) = &mut self.document {
                        if shift {
                            doc.redo();
                        } else {
                            doc.undo();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: PhysicalKey::Code(KeyCode::KeyY),
                        ..
                    },
                ..
            } => {
                if self.modifiers.control_key() || self.modifiers.super_key() {
                    if let Some(doc) = &mut self.document {
                        doc.redo();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Escape),
                        ..
                    },
                ..
            } => {
                event_loop.exit();
            }
            _ => (),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::default();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        match Document::open(&args[1]) {
            Ok(doc) => app.document = Some(doc),
            Err(e) => eprintln!("Failed to load document: {}", e),
        }
    }

    event_loop.run_app(&mut app)?;

    Ok(())
}
