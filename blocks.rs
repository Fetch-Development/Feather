/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020

LESTERRRY AUTHORSHIP
***************************/

use std::env;
use image::{ImageBuffer, RgbImage};
extern crate image;
pub fn set_blocks(rblocks: [u8;64], gblocks: [u8;64], bblocks: [u8;64], marker : usize) -> String{
	let mut img: RgbImage = ImageBuffer::new(500, 500);
	for (x, y, pixel) in img.enumerate_pixels_mut() {
		let val : usize;
		if x > 50 && x < 450 && y > 50 && y < 450 {
			val = (((x / 50) + (8 * ((y / 50) - 1))) - 1) as usize; 
			// if val != recorded{
			// 	println!("{}", val); recorded = val; 
			// }
			*pixel = image::Rgb([rblocks[val], gblocks[val], bblocks[val]]);
		}else{
			*pixel = image::Rgb([255, 255, 255]);
		}
	}
	let dir = env::current_dir().unwrap();
	img.save(dir.to_owned().into_os_string().into_string().unwrap() + "/feather_out_" + &marker.to_string() + ".png").unwrap();
	return dir.to_owned().into_os_string().into_string().unwrap() + "/feather_out_" + &marker.to_string() + ".png";
}