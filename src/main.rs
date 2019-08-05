extern crate libc;

mod color;

use rusttype::{point, Font, Scale};

#[repr(C)]
struct fb {
	len: libc::size_t,
	xres: libc::size_t,
	yres: libc::size_t,
	buf: *mut u16
}

struct Screen<'a> {
	buf: &'a mut [u16],
	xres: usize,
	yres: usize
}

impl<'a> Screen<'a> {
	fn draw_pixel(&mut self, x: usize, y: usize, color: color::Color) {
		if x >= self.xres || y >= self.yres {
			return
		}
		self.buf[y*self.xres + x] = color.0;
	}
}

extern {
	fn get_fb() -> fb;
}

fn draw_rect(x: usize, y: usize, w: usize, l: usize, s: &mut Screen, color: color::Color) {
	for r in y..l {
		for c in x..w {
			s.draw_pixel(c, r, color);
		}
	}
}

const FONT_BYTES: &[u8] = include_bytes!("../fonts/DejaVuSansMono.ttf");

fn draw_text(s: &mut Screen, text: &str) {
	let font = Font::from_bytes(FONT_BYTES).expect("Error constructing font");
	let scale = Scale::uniform(20.0);
	let v_metrics = font.v_metrics(scale);
	let glyphs: Vec<_> = font
		.layout(text, scale, point(20.0, 20.0 + v_metrics.ascent))
		.collect();
	
	for glyph in glyphs {
		if let Some(bounding_box) = glyph.pixel_bounding_box() {
			glyph.draw(|x, y, v| {
				s.draw_pixel((x+bounding_box.min.x as u32) as usize, (y+bounding_box.min.y as u32) as usize, color::Color::from_rgb((255.0*v) as u8, (255.0*v) as u8, (255.0*v) as u8));
			});
		}
	}
}

fn main() {
	let fb = unsafe { get_fb() };
	println!("Got a frame buffer ({}x{}) with {} bytes", fb.xres, fb.yres, fb.len);
	let buffer = unsafe { std::slice::from_raw_parts_mut(fb.buf, fb.len/2) };
	let mut screen = Screen { buf: buffer, xres: fb.xres, yres: fb.yres };
	
	draw_rect(0,0, screen.xres,screen.yres, &mut screen, color::Color(0x0000));
	let text = { 
		let mut args = std::env::args();
		args.next().unwrap();
		args.next().unwrap_or(String::from("Hello, world!"))
	};
	draw_text(&mut screen, &text);
}
