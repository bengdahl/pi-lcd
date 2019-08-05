#[derive(Debug,Clone,Copy)]
pub struct Color(pub u16);

impl Color {
	pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
		let rf: u16 = (((r as f32)/255.0) * 31.0).floor() as u16;
		let gf: u16 = (((g as f32)/255.0) * 63.0).floor() as u16;
		let bf: u16 = (((b as f32)/255.0) * 31.0).floor() as u16;

		Color(rf << 11 | gf << 5 | bf)
	}
}
