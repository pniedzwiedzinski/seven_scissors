use image::{imageops, DynamicImage, Rgba};
use std::fs::read_dir;
use std::path::PathBuf;
use std::env;

fn save_label(img: &mut DynamicImage, x: u32, y: u32, width: u32, height: u32, label: String) {
    let subimg = imageops::crop(img, x, y, width, height).to_image();
    let mut first: Option<Rgba<u8>> = None;
    for pixel in subimg.pixels() {
        if first.is_none() {
            first = Some(*pixel);
        }
        if pixel != &first.unwrap() {
            let dir = env::temp_dir();
            let path = format!("{}/{label}.png", dir.to_str().unwrap());
            println!("Saved to {}", path);
            subimg
                .save(path)
                .unwrap();
            return;
        }
    }
}

fn cut_image(path: &PathBuf) {
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let margin = 5;
    let height = 1400;
    let width = 996;

    let mut img = image::open(path).unwrap();
    save_label(&mut img, 0, margin, width, height, format!("{filename}-0"));
    save_label(&mut img, width, margin, width, height, format!("{filename}-1"));
    save_label(&mut img, 0, height+margin, width, height, format!("{filename}-2"));
    save_label(&mut img, width, height+margin, width, height, format!("{filename}-3"));

}

fn main() {
    let path = env::args().nth(1).expect("no path given");

    let files = read_dir(path).unwrap();

    for file in files {
        cut_image(&file.unwrap().path());
    }


}
