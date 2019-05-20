use std::env;
use image::{GenericImageView, ImageBuffer, Rgba};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = &args[1];

    let img = image::open(input_filename).unwrap();
    let mut test: image::ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(img.width(), img.height());

    let mut unsorted_pixels: Vec<_> = img.pixels().collect();
    let mut sorted_pixels = unsorted_pixels.clone();

    // Sorting by red color in acending order
    unsorted_pixels.sort_by(|a, b| a.2[0].cmp(&b.2[0]));

    for i in 0..unsorted_pixels.len() {
        sorted_pixels[i].2 = unsorted_pixels[i].2;
        test.put_pixel(sorted_pixels[i].0, sorted_pixels[i].1, sorted_pixels[i].2);
    }

    test.save("test.jpg").unwrap();
}
