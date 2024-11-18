use std::{io::Cursor, thread, time::Duration};

use image::ImageReader;
use rascam::{info, SimpleCamera};

enum Colour {
    Red,
    Green,
    Blue,
}

impl Colour {
    pub fn into_idx(&self) -> usize {
	match self {
	    Self::Red => 0,
	    Self::Green => 1,
	    Self::Blue => 2,
	}
    }
}

fn main() {
    let info = info().expect("Could not get camera info!");

    if info.cameras.len() < 1 {
	panic!("Found no cameras connected to the pi!");
    }

    let mut camera = SimpleCamera::new(info.clone()).expect("Could not create camera!");
    camera.activate.expect("Could not activate camera!");

    //give time to position camera properly
    println!("Taking photo in 3 seconds!");
    thread::sleep(Duration::from_secs(3));

    let image = camera.take_one().expect("Could not take image!");
    println!("Photo taken!");

    let decoded = ImageReader::new(Cursor::new(image)).with_guessed_format().expect("Could not guess image format!").decode().expect("Could not decode image!");

    decoded.save("image.jpg").expect("Could not save image!");

    let raw = decoded.into_rgb32f();

    let (width, height) = raw.dimensions();

    let pixels = (width * height) as f32;

    let mut total_r: f32 = 0.;
    let mut total_g: f32 = 0.;
    let mut total_b: f32 = 0.;
    
    for (x, y, pixel) in raw.enumerate_pixels() {
	let rgb = pixel.0;
	total_r += rgb[0];
	total_g += rgb[1];
	total_b += rgb[2];
    }

    let average_r = total_r / pixels;
    let average_g = total_g / pixels;
    let average_b = total_b / pixels;

    let block_colour = match (average_r > average_g && average_r > average_b, average_g > average_r && average_g > average_b, average_b > average_r && average_b > average_g) {
	(true, false, false) => {
	    Colour::Red
	},
	(false, true, false) => {
	    Colour::Green
	},
	(false, false, true) => {
	    Colour::Blue
	},
	_ => unreachable!(),
    };

    let checked: f32 = 0.;
    
    for (x, y, pixel) in raw.enumerate_pixels() {
	let mut rgb = Vec::from(pixel.rgb);
	let block_channel = rgb.remove(block_colour.into_idx());
	let other_sum = rgb[0] + rgb[1];
	if block_channel > other_sum {
	    checked += 1;
	}
    }

    let fraction_covered = checked / pixels;
    let percent_covered = fraction_covered * 100.;
    println!("Percent of image covered by lego: {percent_covered}");
}
