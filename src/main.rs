use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use num_complex;
mod map;
fn main() {
    let imgx = 20;
    let imgy = 20;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut ocean_map = map::Map::new(imgx as usize, imgy as usize);
    ocean_map.render(&mut imgbuf);

    let scramble_count = 50000;
    for i in 0..scramble_count {
        ocean_map.scramble();
        println!("{}", i);
    }
    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("map.png").unwrap();
}
