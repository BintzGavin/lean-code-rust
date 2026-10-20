use rusttype::{point, Font, Scale};
use softbuffer::{Context, Surface};
use std::num::NonZeroU32;
use std::rc::Rc;
use winit::window::Window;

pub struct Renderer {
    surface: Surface<Rc<Window>, Rc<Window>>,
    font: Font<'static>,
    width: u32,
    height: u32,
    scroll_y: f32,
}

impl Renderer {
    pub fn new(window: Rc<Window>) -> Result<Self, Box<dyn std::error::Error>> {
        let context = Context::new(window.clone()).unwrap();
        let surface = Surface::new(&context, window.clone()).unwrap();

        let font_data = include_bytes!("../assets/JetBrainsMono-Regular.ttf");
        let font = Font::try_from_bytes(font_data as &[u8]).expect("Error constructing Font");

        Ok(Self {
            surface,
            font,
            width: 0,
            height: 0,
            scroll_y: 0.0,
        })
    }

    pub fn hit_test(&self, text: &str, target_x: f32, target_y: f32) -> usize {
        if self.width == 0 || self.height == 0 {
            return 0;
        }
        let scale = Scale::uniform(24.0);
        let v_metrics = self.font.v_metrics(scale);
        let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        let mut y = 20.0 + v_metrics.ascent - self.scroll_y;
        let mut x = 20.0;

        let mut char_count = 0;

        for c in text.chars() {
            let advance = if c == '\n' {
                0.0
            } else {
                self.font.glyph(c).scaled(scale).h_metrics().advance_width
            };

            let line_bottom = y - v_metrics.ascent + line_height;

            if target_y <= line_bottom {
                // Click is on this line
                if target_x <= x + (advance / 2.0) {
                    return char_count;
                }
                if c == '\n' {
                    return char_count;
                }
            }

            if c == '\n' {
                x = 20.0;
                y += line_height;
            } else {
                x += advance;
            }
            char_count += 1;
        }

        char_count
    }


    pub fn scroll(&mut self, delta: f32) {
        self.scroll_y = (self.scroll_y - delta).max(0.0);
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        if width > 0 && height > 0 {
            self.surface
                .resize(
                    NonZeroU32::new(width).unwrap(),
                    NonZeroU32::new(height).unwrap(),
                )
                .unwrap();
        }
    }

    pub fn draw(&mut self, text: &str, cursor_pos: usize, selection_anchor: Option<usize>) {
        if self.width == 0 || self.height == 0 {
            return;
        }

        let mut buffer = self.surface.buffer_mut().unwrap();

        let width = self.width as usize;
        let height = self.height as usize;
        let mut pixels = vec![0xFF1E1E1E; width * height]; // background #1e1e1e

        let scale = Scale::uniform(24.0);
        let v_metrics = self.font.v_metrics(scale);
        let line_height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

        let mut cursor_doc_y = 20.0;
        let mut doc_char_count = 0;
        for c in text.chars() {
            if doc_char_count == cursor_pos {
                break;
            }
            if c == '\n' {
                cursor_doc_y += line_height;
            }
            doc_char_count += 1;
        }

        let margin_top = 20.0;
        let margin_bottom = 20.0;

        if cursor_doc_y < self.scroll_y + margin_top {
            self.scroll_y = (cursor_doc_y - margin_top).max(0.0);
        }
        if cursor_doc_y + line_height > self.scroll_y + (self.height as f32) - margin_bottom {
            self.scroll_y = cursor_doc_y + line_height - (self.height as f32) + margin_bottom;
        }

        let mut y = 20.0 + v_metrics.ascent - self.scroll_y;
        let mut x = 20.0;
        let mut cursor_x = 20.0;
        let mut cursor_y = 20.0;

        let mut char_count = 0;
        let mut cursor_drawn = false;

        if cursor_pos == 0 {
            cursor_x = x;
            cursor_y = y - v_metrics.ascent;
            cursor_drawn = true;
        }

        let sel_start = selection_anchor.map(|a| std::cmp::min(a, cursor_pos));
        let sel_end = selection_anchor.map(|a| std::cmp::max(a, cursor_pos));

        for c in text.chars() {
            let advance = if c == '\n' {
                0.0
            } else {
                self.font.glyph(c).scaled(scale).h_metrics().advance_width
            };

            let is_selected = if let (Some(start), Some(end)) = (sel_start, sel_end) {
                char_count >= start && char_count < end
            } else {
                false
            };

            if is_selected {
                let rect_x = x as i32;
                let rect_y = (y - v_metrics.ascent) as i32;
                let rect_w = if c == '\n' { 10.0 } else { advance } as i32;
                let rect_h = (v_metrics.ascent - v_metrics.descent) as i32;

                for py in 0..rect_h {
                    for px in 0..rect_w {
                        let draw_x = rect_x + px;
                        let draw_y = rect_y + py;
                        if draw_x >= 0
                            && draw_x < width as i32
                            && draw_y >= 0
                            && draw_y < height as i32
                        {
                            pixels[(draw_y as usize) * width + (draw_x as usize)] = 0xFF264F78;
                            // Selection color
                        }
                    }
                }
            }

            if c == '\n' {
                x = 20.0;
                y += v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;
            } else {
                let glyph = self.font.glyph(c).scaled(scale).positioned(point(x, y));
                if let Some(bb) = glyph.pixel_bounding_box() {
                    glyph.draw(|gx, gy, v| {
                        let px = bb.min.x + gx as i32;
                        let py = bb.min.y + gy as i32;
                        if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                            let idx = (py as usize) * width + (px as usize);
                            let bg = pixels[idx];
                            let bg_r = ((bg >> 16) & 0xFF) as f32;
                            let bg_g = ((bg >> 8) & 0xFF) as f32;
                            let bg_b = (bg & 0xFF) as f32;

                            let fg_r = 220.0;
                            let fg_g = 220.0;
                            let fg_b = 220.0;

                            let alpha = v;
                            let inv_alpha = 1.0 - alpha;

                            let out_r = (fg_r * alpha + bg_r * inv_alpha) as u32;
                            let out_g = (fg_g * alpha + bg_g * inv_alpha) as u32;
                            let out_b = (fg_b * alpha + bg_b * inv_alpha) as u32;

                            pixels[idx] = 0xFF000000 | (out_r << 16) | (out_g << 8) | out_b;
                        }
                    });
                }
                x += advance;
            }
            char_count += 1;

            if char_count == cursor_pos {
                cursor_x = x;
                cursor_y = y - v_metrics.ascent;
                cursor_drawn = true;
            }
        }

        if !cursor_drawn {
            cursor_x = x;
            cursor_y = y - v_metrics.ascent;
        }

        // cursor rect
        let cursor_w = 2;
        let cursor_h = (v_metrics.ascent - v_metrics.descent) as i32;
        let cx = cursor_x as i32;
        let cy = cursor_y as i32;

        for py in 0..cursor_h {
            for px in 0..cursor_w {
                let draw_x = cx + px;
                let draw_y = cy + py;
                if draw_x >= 0 && draw_x < width as i32 && draw_y >= 0 && draw_y < height as i32 {
                    pixels[(draw_y as usize) * width + (draw_x as usize)] = 0xFFCCCCCC;
                    // Cursor color
                }
            }
        }

        for (i, p) in pixels.iter().enumerate() {
            buffer[i] = *p;
        }

        buffer.present().unwrap();
    }
}
