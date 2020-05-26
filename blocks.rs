/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020

LESTERRRY AUTHORSHIP
***************************/

use image::{ImageBuffer, RgbImage};
extern crate image;
pub fn set_blocks(rblocks: [u8;64], gblocks: [u8;64], bblocks: [u8;64]){
	/* let mut rblocks: [u8; 64] = [100; 64]
	[10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10];
	rblocks[63] = 255; 
	let mut gblocks: [u8; 64] = [100; 64]
	[10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10];
	gblocks[62] = 255; 
	let mut bblocks: [u8; 64] = [100; 64]
	[10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,10,
	10,10,10,10]; 
	bblocks[61] = 255; 
	*/
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
	img.save("feather_out.png").unwrap();
}