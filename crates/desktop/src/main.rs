use lean_editor_core::Document;
use std::env;
use std::rc::Rc;
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, MouseButton, MouseScrollDelta, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::{Key, KeyCode, ModifiersState, NamedKey, PhysicalKey},
    window::{Window, WindowId},
};

use arboard::Clipboard;
mod render;
use render::Renderer;

#[derive(Default)]
struct App {
    window: Option<Rc<Window>>,
    renderer: Option<Renderer>,
    document: Option<Document>,
    modifiers: ModifiersState,
    cursor_pos: usize,
    selection_anchor: Option<usize>,
    clipboard: Option<Clipboard>,
    mouse_pos: (f32, f32),
}

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

    fn request_redraw(&self) {
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = Window::default_attributes().with_title("Lean Editor");
            let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
            self.window = Some(window.clone());
            self.renderer = Some(Renderer::new(window).unwrap());
            self.update_title();
            self.request_redraw();
            match Clipboard::new() {
                Ok(cb) => self.clipboard = Some(cb),
                Err(e) => eprintln!("Failed to initialize clipboard: {e}"),
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let mut needs_redraw = false;

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
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                    needs_redraw = true;
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    let text = self.document.as_ref().map(|d| d.text()).unwrap_or("");
                    renderer.draw(text, self.cursor_pos, self.selection_anchor);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_pos = (position.x as f32, position.y as f32);
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let (Some(renderer), Some(doc)) = (&self.renderer, &self.document) {
                    let clicked_pos =
                        renderer.hit_test(doc.text(), self.mouse_pos.0, self.mouse_pos.1);
                    if self.modifiers.shift_key() {
                        if self.selection_anchor.is_none() {
                            self.selection_anchor = Some(self.cursor_pos);
                        }
                    } else {
                        self.selection_anchor = None;
                    }
                    self.cursor_pos = clicked_pos;
                    needs_redraw = true;
                } else if let Some(renderer) = &self.renderer {
                    // Handle empty doc (no doc loaded)
                    let clicked_pos = renderer.hit_test("", self.mouse_pos.0, self.mouse_pos.1);
                    self.cursor_pos = clicked_pos;
                    self.selection_anchor = None;
                    needs_redraw = true;
                }
            }
            WindowEvent::ModifiersChanged(modifiers) => {
                self.modifiers = modifiers.state();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if (c == "a" || c == "A")
                && (self.modifiers.control_key() || self.modifiers.super_key()) =>
            {
                self.selection_anchor = Some(0);
                self.cursor_pos = self.doc_char_count();
                needs_redraw = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if (c == "s" || c == "S")
                && (self.modifiers.control_key() || self.modifiers.super_key()) =>
            {
                if let Some(doc) = &mut self.document {
                    if let Err(e) = doc.save() {
                        eprintln!("Failed to save document: {}", e);
                    }
                    self.update_title();
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
                        needs_redraw = true;
                        self.update_title();
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
                        needs_redraw = true;
                        self.update_title();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if (c == "c" || c == "C")
                && (self.modifiers.control_key() || self.modifiers.super_key()) =>
            {
                if let (Some(doc), Some(cb)) = (&self.document, &mut self.clipboard) {
                    if let Some(anchor) = self.selection_anchor {
                        let start = std::cmp::min(self.cursor_pos, anchor);
                        let end = std::cmp::max(self.cursor_pos, anchor);
                        if start < end {
                            let text = doc.text();
                            let selected_text: String =
                                text.chars().skip(start).take(end - start).collect();
                            if let Err(e) = cb.set_text(selected_text) {
                                eprintln!("Failed to copy to clipboard: {}", e);
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if (c == "x" || c == "X")
                && (self.modifiers.control_key() || self.modifiers.super_key()) =>
            {
                if let (Some(doc), Some(cb)) = (&mut self.document, &mut self.clipboard) {
                    if let Some(anchor) = self.selection_anchor {
                        let start = std::cmp::min(self.cursor_pos, anchor);
                        let end = std::cmp::max(self.cursor_pos, anchor);
                        if start < end {
                            let text = doc.text();
                            let selected_text: String =
                                text.chars().skip(start).take(end - start).collect();
                            if let Err(e) = cb.set_text(selected_text) {
                                eprintln!("Failed to copy to clipboard: {}", e);
                            }

                            doc.edit_range(start, end, "");
                            self.cursor_pos = start;
                            self.selection_anchor = None;
                            needs_redraw = true;
                            self.update_title();
                        }
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
                    needs_redraw = true;
                    self.update_title();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Delete),
                        ..
                    },
                ..
            } => {
                if let Some(doc) = &mut self.document {
                    let char_count = doc.text().chars().count();
                    if let Some(anchor) = self.selection_anchor {
                        let start = std::cmp::min(self.cursor_pos, anchor);
                        let end = std::cmp::max(self.cursor_pos, anchor);
                        if start < end {
                            doc.edit_range(start, end, "");
                            self.cursor_pos = start;
                        } else if self.cursor_pos < char_count {
                            doc.edit_range(self.cursor_pos, self.cursor_pos + 1, "");
                        }
                        self.selection_anchor = None;
                    } else if self.cursor_pos < char_count {
                        doc.edit_range(self.cursor_pos, self.cursor_pos + 1, "");
                    }
                    needs_redraw = true;
                    self.update_title();
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
                    needs_redraw = true;
                    self.update_title();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Tab),
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
                    doc.edit_range(start, end, "    ");
                    self.cursor_pos = start + 4;
                    self.selection_anchor = None;
                    needs_redraw = true;
                    self.update_title();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Character(c),
                        ..
                    },
                ..
            } if (c == "v" || c == "V")
                && (self.modifiers.control_key() || self.modifiers.super_key()) =>
            {
                if let (Some(doc), Some(cb)) = (&mut self.document, &mut self.clipboard) {
                    if let Ok(text_to_paste) = cb.get_text() {
                        let (start, end) = if let Some(anchor) = self.selection_anchor {
                            (
                                std::cmp::min(self.cursor_pos, anchor),
                                std::cmp::max(self.cursor_pos, anchor),
                            )
                        } else {
                            (self.cursor_pos, self.cursor_pos)
                        };
                        let char_count = text_to_paste.chars().count();
                        doc.edit_range(start, end, &text_to_paste);
                        self.cursor_pos = start + char_count;
                        self.selection_anchor = None;
                        needs_redraw = true;
                        self.update_title();
                    }
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
                            needs_redraw = true;
                            self.update_title();
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::Home),
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
                    let (line, _) = get_line_col(text, self.cursor_pos);
                    self.cursor_pos = get_pos_from_line_col(text, line, 0);
                }
                needs_redraw = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::End),
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
                    let (line, _) = get_line_col(text, self.cursor_pos);
                    let start_pos = get_pos_from_line_col(text, line, 0);
                    self.cursor_pos = start_pos
                        + text[start_pos..]
                            .find('\n')
                            .unwrap_or(text[start_pos..].len());
                }
                needs_redraw = true;
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
                needs_redraw = true;
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
                needs_redraw = true;
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
                needs_redraw = true;
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
                needs_redraw = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::PageUp),
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
                    let page_lines = self
                        .renderer
                        .as_ref()
                        .map(|r| r.lines_per_page())
                        .unwrap_or(20);
                    let new_line = line.saturating_sub(page_lines.saturating_sub(1).max(1));
                    self.cursor_pos = get_pos_from_line_col(text, new_line, col);
                }
                needs_redraw = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        logical_key: Key::Named(NamedKey::PageDown),
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
                    let page_lines = self
                        .renderer
                        .as_ref()
                        .map(|r| r.lines_per_page())
                        .unwrap_or(20);
                    let new_line = line + page_lines.saturating_sub(1).max(1);
                    self.cursor_pos = get_pos_from_line_col(text, new_line, col);
                }
                needs_redraw = true;
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if let Some(renderer) = &mut self.renderer {
                    let scroll_amount = match delta {
                        MouseScrollDelta::LineDelta(_, y) => y * 20.0,
                        MouseScrollDelta::PixelDelta(pos) => pos.y as f32,
                    };
                    renderer.scroll(scroll_amount);
                    needs_redraw = true;
                }
            }
            _ => (),
        }

        if needs_redraw {
            self.request_redraw();
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
            Ok(doc) => {
                let char_count = doc.text().chars().count();
                app.document = Some(doc);
                app.cursor_pos = char_count;
            }
            Err(e) => eprintln!("Failed to load document: {}", e),
        }
    } else {
        // Create empty document by default if none provided
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join("untitled.txt");
        if !temp_file.exists() {
            std::fs::write(&temp_file, "").unwrap();
        }
        match Document::open(temp_file.to_str().unwrap()) {
            Ok(doc) => {
                app.document = Some(doc);
                app.cursor_pos = 0;
            }
            Err(e) => eprintln!("Failed to load default document: {}", e),
        }
    }

    event_loop.run_app(&mut app)?;

    Ok(())
}
