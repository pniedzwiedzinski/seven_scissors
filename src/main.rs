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
            subimg
                .save(format!("/tmp/{label}.png"))
                .unwrap();
            return;
        }
    }
}

fn cut_image(path: &PathBuf) {
    let filename = path.file_stem().unwrap().to_str().unwrap();
    let top_pad = 30;
    let margin = 15;
    let height = 900;
    let width = 2000;

    let mut img = image::open(path).unwrap();
    save_label(&mut img, 0, top_pad+margin, width, height, format!("{filename}-0"));
    save_label(&mut img, 0, height+top_pad, width, height, format!("{filename}-1"));
    save_label(&mut img, 0, height+height+margin, width, height, format!("{filename}-2"));

}

fn main() {
    let path = env::args().nth(1).expect("no path given");

    let files = read_dir(path).unwrap();

    for file in files {
        cut_image(&file.unwrap().path());
    }


}
