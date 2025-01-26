mod constants;
use core::{
    fmt::{self, Write},
    ptr,
};
use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use constants::font_constants;
use constants::font_constants::{BACKUP_CHAR, CHAR_RASTER_HEIGHT, FONT_WEIGHT};
use noto_sans_mono_bitmap::{get_raster, RasterizedChar};

/// Additional vertical space between lines
const LINE_SPACING: usize = 2;
/// Additional horizontal space between characters.
const LETTER_SPACING: usize = 0;
/// Padding from the border. Prevent that font is too close to border.
const BORDER_PADDING: usize = 1;

/// Returns the raster of the given char or the raster of [font_constants::BACKUP_CHAR].
fn get_char_raster(c: char) -> RasterizedChar {
    fn get(c: char) -> Option<RasterizedChar> {
        get_raster(c, FONT_WEIGHT, CHAR_RASTER_HEIGHT)
    }
    get(c).unwrap_or_else(|| get(BACKUP_CHAR).expect("Should get raster of backup char."))
}

/// Allows logging text to a pixel-based framebuffer.
pub struct FrameBufferWriter {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
    current_color: [u8; 3],
    default_color: [u8; 3],
    processing_escape: bool,
}

impl FrameBufferWriter {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
            current_color: [255, 255, 255],
            default_color: [255, 255, 255],
            processing_escape: false,
        };
        logger.clear();
        logger
    }

    /// Manually set cursor position.
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        if x < self.width() && y < self.height() {
            self.x_pos = x;
            self.y_pos = y;
        } else {
            panic!("OUT OF BOUNDS!!");
        }
    }

    fn newline(&mut self) {
        self.y_pos += font_constants::CHAR_RASTER_HEIGHT.val() + LINE_SPACING;
        self.carriage_return();
    }

    fn carriage_return(&mut self) {
        self.x_pos = BORDER_PADDING;
    }

    /// Erases all text on the screen. Resets `x_pos` and `y_pos`.
    pub fn clear(&mut self) {
        self.x_pos = BORDER_PADDING;
        self.y_pos = BORDER_PADDING;
        self.framebuffer.fill(0);
    }

    fn width(&self) -> usize {
        self.info.width
    }

    fn height(&self) -> usize {
        self.info.height
    }

    /// Writes a single char to the framebuffer, handling special control characters.
    fn write_char(&mut self, c: char) {
        if self.processing_escape {
            // Handle escape sequences
            match c {
                'c' => {
                    // Set the current color to blue
                    self.current_color = [0, 0, 255]; // RGB for blue
                    self.processing_escape = false; // Exit escape sequence mode
                }
                _ => {
                    // Invalid escape sequence, write both characters
                    self.processing_escape = false;
                    self.write_char('\\'); // Write the '\' character
                    self.write_char(c); // Write the invalid escape character
                }
            }
        } else {
            match c {
                '\\' => {
                    // Start processing an escape sequence
                    self.processing_escape = true;
                }
                ' ' => {
                    // Reset color to default on encountering a space
                    self.current_color = self.default_color;
                    self.write_rendered_char(get_char_raster(c), self.current_color);
                }
                '\n' => self.newline(),
                '\t' => {
                    // Insert four spaces for a tab character
                    for _ in 0..4 {
                        self.write_char(' '); // Write four spaces to the framebuffer
                    }
                }
                '\r' => self.carriage_return(),
                _ => {
                    let new_xpos = self.x_pos + font_constants::CHAR_RASTER_WIDTH;
                    if new_xpos >= self.width() {
                        self.newline();
                    }

                    // Render the character using the current color
                    self.write_rendered_char(get_char_raster(c), self.current_color);
                }
            }
        }
    }

    /// Updates `x_pos`.
    fn write_rendered_char(&mut self, rendered_char: RasterizedChar, color: [u8; 3]) {
        for (y, row) in rendered_char.raster().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                if *byte > 0 {
                    // Non-zero byte means the pixel should be drawn
                    self.write_pixel(self.x_pos + x, self.y_pos + y, color);
                }
            }
        }
        // Move the cursor forward after writing the character
        self.x_pos += rendered_char.width() + LETTER_SPACING;
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: [u8; 3]) {
        if x >= self.width() || y >= self.height() {
            return; // Ignore out-of-bounds pixels
        }

        let pixel_offset = y * self.info.stride + x; // Calculate the pixel's offset in the framebuffer
        let bytes_per_pixel = self.info.bytes_per_pixel;

        // Set the color based on the pixel format
        let final_color = match self.info.pixel_format {
            PixelFormat::Rgb => [color[0], color[1], color[2], 0],
            PixelFormat::Bgr => [color[2], color[1], color[0], 0],
            PixelFormat::U8 => [if color[0] > 128 { 0xff } else { 0 }, 0, 0, 0],
            other => {
                // Unsupported pixel format: fall back to RGB and panic for unsupported cases
                self.info.pixel_format = PixelFormat::Rgb;
                panic!("Unsupported pixel format: {:?}", other);
            }
        };

        // Calculate the byte offset for this pixel
        let byte_offset = pixel_offset * bytes_per_pixel;

        // Copy the color into the framebuffer
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&final_color[..bytes_per_pixel]);

        // Ensure the write happens immediately by using a volatile read
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for FrameBufferWriter {}
unsafe impl Sync for FrameBufferWriter {}

impl Write for FrameBufferWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
