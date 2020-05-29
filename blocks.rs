/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020

LESTERRRY AUTHORSHIP
***************************/

use std::env;
use image::{ImageBuffer, RgbImage};
extern crate image;
use imageproc::drawing::draw_text_mut;
use rusttype::{FontCollection, Scale};

pub fn set_blocks(rblocks: [u8;64], gblocks: [u8;64], bblocks: [u8;64], marker: usize, header: String) -> String {
	let mut img: RgbImage = ImageBuffer::new(1000, 1000);
	for (x, y, pixel) in img.enumerate_pixels_mut() {
		let val : usize;
		if x > 100 && x < 900 && y > 100 && y < 900 {
			val = (((x / 100) + (8 * ((y / 100) - 1))) - 1) as usize; 
			*pixel = image::Rgb([rblocks[val], gblocks[val], bblocks[val]]);
		}else{
			*pixel = image::Rgb([255, 255, 255]);
		}
	}
	let font = Vec::from(include_bytes!("Futura.ttf") as &[u8]);
	let font = FontCollection::from_bytes(font)
		.unwrap()
		.into_font()
		.unwrap();

	let headscale = Scale {
		x: 40.0,
		y: 40.0,
	};
	let footscale = Scale {
		x: 30.0,
		y: 30.0,
	};

	draw_text_mut(
		&mut img,
		image::Rgb([0, 0, 0]),
		40,
		40,
		headscale,
		&font,
		&header,
	);
	draw_text_mut(
		&mut img,
		image::Rgb([0, 0, 0]),
		650,
		940,
		footscale,
		&font,
		"FETCH DEVELOPMENT",
	);

	let dir = env::current_dir().unwrap();
	img.save(dir.to_owned().into_os_string().into_string().unwrap() + "/feather_out_" + &marker.to_string() + ".png").unwrap();
	return dir.to_owned().into_os_string().into_string().unwrap() + "/feather_out_" + &marker.to_string() + ".png";
}