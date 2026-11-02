/// Pixel-based text renderer using the bootloader GOP framebuffer + 8×8 font.
use bootloader_api::info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use core::fmt;
use font8x8::legacy::BASIC_LEGACY;
use spin::Mutex;

const CHAR_W: usize = 8;
const CHAR_H: usize = 8;
const FG: (u8, u8, u8) = (180, 255, 140); // light green
const BG: (u8, u8, u8) = (10, 10, 10);     // near-black

// ── FramebufferWriter ─────────────────────────────────────────────────────────

pub struct FramebufferWriter {
    ptr: *mut u8,
    len: usize,
    info: FrameBufferInfo,
    col: usize, // current character column
    row: usize, // current character row
}

unsafe impl Send for FramebufferWriter {}

impl FramebufferWriter {
    fn cols(&self) -> usize {
        self.info.width / CHAR_W
    }
    fn rows(&self) -> usize {
        self.info.height / CHAR_H
    }

    fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        let stride = self.info.stride;
        let bpp = self.info.bytes_per_pixel;
        let offset = y * stride * bpp + x * bpp;
        if offset + bpp > self.len {
            return;
        }
        // SAFETY: offset + bpp is within the framebuffer slice we own.
        let bytes = unsafe { core::slice::from_raw_parts_mut(self.ptr.add(offset), bpp) };
        match self.info.pixel_format {
            PixelFormat::Rgb => {
                if bpp >= 3 {
                    bytes[0] = r;
                    bytes[1] = g;
                    bytes[2] = b;
                }
            }
            PixelFormat::Bgr => {
                if bpp >= 3 {
                    bytes[0] = b;
                    bytes[1] = g;
                    bytes[2] = r;
                }
            }
            _ => {
                // Grayscale approximation
                let luma = (r as u16 * 77 + g as u16 * 150 + b as u16 * 29) >> 8;
                bytes[0] = luma as u8;
            }
        }
    }

    fn draw_char(&mut self, c: u8, col: usize, row: usize) {
        let idx = (c as usize).min(127);
        let glyph = BASIC_LEGACY[idx];
        let px = col * CHAR_W;
        let py = row * CHAR_H;
        for (dy, &bits) in glyph.iter().enumerate() {
            for dx in 0..CHAR_W {
                // font8x8: bit 0 = leftmost pixel
                let lit = (bits >> dx) & 1 != 0;
                let (r, g, b) = if lit { FG } else { BG };
                self.set_pixel(px + dx, py + dy, r, g, b);
            }
        }
    }

    fn clear_screen(&mut self) {
        let (r, g, b) = BG;
        let h = self.info.height;
        let w = self.info.width;
        for y in 0..h {
            for x in 0..w {
                self.set_pixel(x, y, r, g, b);
            }
        }
    }

    fn scroll_up(&mut self) {
        let bpp = self.info.bytes_per_pixel;
        let stride = self.info.stride;
        let row_bytes = CHAR_H * stride * bpp;
        let total = self.rows() * row_bytes;
        unsafe {
            core::ptr::copy(
                self.ptr.add(row_bytes),
                self.ptr,
                total - row_bytes,
            );
            // Clear last row
            core::ptr::write_bytes(self.ptr.add(total - row_bytes), 0, row_bytes);
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.col = 0;
                if self.row + 1 < self.rows() {
                    self.row += 1;
                } else {
                    self.scroll_up();
                }
            }
            b'\r' => {
                self.col = 0;
            }
            b'\x08' => {
                // backspace
                if self.col > 0 {
                    self.col -= 1;
                    self.draw_char(b' ', self.col, self.row);
                }
            }
            b => {
                if self.col >= self.cols() {
                    self.col = 0;
                    if self.row + 1 < self.rows() {
                        self.row += 1;
                    } else {
                        self.scroll_up();
                    }
                }
                self.draw_char(b, self.col, self.row);
                self.col += 1;
            }
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for b in s.bytes() {
            match b {
                0x20..=0x7E | b'\n' | b'\r' | b'\x08' => self.write_byte(b),
                _ => self.write_byte(b'?'),
            }
        }
    }

    pub fn clear(&mut self) {
        self.clear_screen();
        self.col = 0;
        self.row = 0;
    }
}

impl fmt::Write for FramebufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

// ── Global ────────────────────────────────────────────────────────────────────

static FB: Mutex<Option<FramebufferWriter>> = Mutex::new(None);

pub fn init(fb: &mut FrameBuffer) {
    let info = fb.info();
    let buf = fb.buffer_mut();
    let ptr = buf.as_mut_ptr();
    let len = buf.len();
    let mut writer = FramebufferWriter {
        ptr,
        len,
        info,
        col: 0,
        row: 0,
    };
    writer.clear_screen();
    *FB.lock() = Some(writer);
}

pub fn _print(args: fmt::Arguments) {
    use fmt::Write;
    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(w) = FB.lock().as_mut() {
            w.write_fmt(args).ok();
        }
    });
}

pub fn clear() {
    x86_64::instructions::interrupts::without_interrupts(|| {
        if let Some(w) = FB.lock().as_mut() {
            w.clear();
        }
    });
}

pub fn available() -> bool {
    FB.lock().is_some()
}

#[macro_export]
macro_rules! fb_print {
    ($($arg:tt)*) => { $crate::framebuffer::_print(format_args!($($arg)*)) };
}

#[macro_export]
macro_rules! fb_println {
    ()             => { $crate::fb_print!("\n") };
    ($($arg:tt)*) => { $crate::fb_print!("{}\n", format_args!($($arg)*)) };
}
