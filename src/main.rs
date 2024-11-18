use std::{thread, time::Duration};

use image::ImageReader;
use rascam::{info, SimpleCamera};

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

    let decoded = ImageReader::new(image).with_guessed_format().decode();
}
