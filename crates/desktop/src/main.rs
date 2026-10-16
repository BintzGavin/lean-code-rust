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

impl App {
    fn update_title(&self) {
        if let Some(window) = &self.window {
            let is_dirty = self
                .document
                .as_ref()
                .map(|doc| doc.is_dirty())
                .unwrap_or(false);
            if is_dirty {
                window.set_title("Lean Editor *");
            } else {
                window.set_title("Lean Editor");
            }
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("Lean Editor");
            let window = event_loop.create_window(window_attributes).unwrap();
            self.window = Some(window);
            self.update_title();
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
                let should_close = self
                    .document
                    .as_ref()
                    .map(|doc| doc.request_close() == lean_editor_core::CloseOutcome::Closed)
                    .unwrap_or(true);
                if should_close {
                    event_loop.exit();
                }
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
                let should_close = self
                    .document
                    .as_ref()
                    .map(|doc| doc.request_close() == lean_editor_core::CloseOutcome::Closed)
                    .unwrap_or(true);
                if should_close {
                    event_loop.exit();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Backspace),
                        ..
                    },
                ..
            } => {
                if let Some(doc) = &mut self.document {
                    let mut current_text = doc.text().to_string();
                    current_text.pop();
                    doc.replace_text(current_text);
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Enter),
                        ..
                    },
                ..
            } => {
                if let Some(doc) = &mut self.document {
                    let current_text = doc.text().to_string();
                    doc.replace_text(format!("{}\n", current_text));
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        text: Some(t),
                        ..
                    },
                ..
            } => {
                if !self.modifiers.control_key() && !self.modifiers.super_key() {
                    let text_str: &str = t.as_ref();
                    if let Some(doc) = &mut self.document {
                        let current_text = doc.text().to_string();
                        doc.replace_text(format!("{}{}", current_text, text_str));
                    }
                }
            }
            _ => (),
        }
        self.update_title();
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
