fn get_line_col(text: &str, cursor_pos: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for (i, c) in text.chars().enumerate() {
        if i == cursor_pos {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

fn get_pos_from_line_col(text: &str, target_line: usize, target_col: usize) -> usize {
    let mut line = 0;
    let mut col = 0;
    let mut pos = 0;
    for c in text.chars() {
        if line == target_line && col == target_col {
            break;
        }
        if c == '\n' {
            if line == target_line {
                break;
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
        pos += 1;
    }
    pos
}

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
    cursor_pos: usize,
    selection_anchor: Option<usize>,
}

impl App {
    fn doc_char_count(&self) -> usize {
        self.document
            .as_ref()
            .map(|doc| doc.text().chars().count())
            .unwrap_or(0)
    }

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
                    if let Some(anchor) = self.selection_anchor {
                        let start = std::cmp::min(self.cursor_pos, anchor);
                        let end = std::cmp::max(self.cursor_pos, anchor);
                        if start < end {
                            doc.edit_range(start, end, "");
                            self.cursor_pos = start;
                        } else if self.cursor_pos > 0 {
                            doc.edit_range(self.cursor_pos - 1, self.cursor_pos, "");
                            self.cursor_pos -= 1;
                        }
                        self.selection_anchor = None;
                    } else if self.cursor_pos > 0 {
                        doc.edit_range(self.cursor_pos - 1, self.cursor_pos, "");
                        self.cursor_pos -= 1;
                    }
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
                    let (start, end) = if let Some(anchor) = self.selection_anchor {
                        (
                            std::cmp::min(self.cursor_pos, anchor),
                            std::cmp::max(self.cursor_pos, anchor),
                        )
                    } else {
                        (self.cursor_pos, self.cursor_pos)
                    };
                    doc.edit_range(start, end, "\n");
                    self.cursor_pos = start + 1;
                    self.selection_anchor = None;
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
                if !self.modifiers.control_key()
                    && !self.modifiers.super_key()
                    && !self.modifiers.alt_key()
                {
                    let text_str: &str = t.as_ref();
                    if text_str.chars().any(|c| !c.is_control()) {
                        if let Some(doc) = &mut self.document {
                            let (start, end) = if let Some(anchor) = self.selection_anchor {
                                (
                                    std::cmp::min(self.cursor_pos, anchor),
                                    std::cmp::max(self.cursor_pos, anchor),
                                )
                            } else {
                                (self.cursor_pos, self.cursor_pos)
                            };
                            let char_count = text_str.chars().count();
                            doc.edit_range(start, end, text_str);
                            self.cursor_pos = start + char_count;
                            self.selection_anchor = None;
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::ArrowLeft),
                        ..
                    },
                ..
            } => {
                if self.modifiers.shift_key() {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::ArrowRight),
                        ..
                    },
                ..
            } => {
                if self.modifiers.shift_key() {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if self.cursor_pos < self.doc_char_count() {
                    self.cursor_pos += 1;
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::ArrowUp),
                        ..
                    },
                ..
            } => {
                if self.modifiers.shift_key() {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if let Some(doc) = &self.document {
                    let text = doc.text();
                    let (line, col) = get_line_col(text, self.cursor_pos);
                    if line > 0 {
                        self.cursor_pos = get_pos_from_line_col(text, line - 1, col);
                    } else {
                        self.cursor_pos = 0;
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::ArrowDown),
                        ..
                    },
                ..
            } => {
                if self.modifiers.shift_key() {
                    if self.selection_anchor.is_none() {
                        self.selection_anchor = Some(self.cursor_pos);
                    }
                } else {
                    self.selection_anchor = None;
                }
                if let Some(doc) = &self.document {
                    let text = doc.text();
                    let (line, col) = get_line_col(text, self.cursor_pos);
                    self.cursor_pos = get_pos_from_line_col(text, line + 1, col);
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
            Ok(doc) => {
                let char_count = doc.text().chars().count();
                app.document = Some(doc);
                app.cursor_pos = char_count;
            }
            Err(e) => eprintln!("Failed to load document: {}", e),
        }
    }

    event_loop.run_app(&mut app)?;

    Ok(())
}
