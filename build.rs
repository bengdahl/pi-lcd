extern crate cc;

fn main() {
	cc::Build::new()
		.file("src/fb.c")
		.compile("libfb.a");
}
