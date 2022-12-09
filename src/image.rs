use std::{fs::File, io::Write};
use tar::Archive;

const IMAGE_DIR: &str = concat!(env!("HOME"), "/.rocker", "/images");
const FILE_PATH: &str = concat!(env!("HOME"), "/.rocker", "/images", "/filesystem.tar");

fn download_image_tar() {
    let url = "https://github.com/0x0elliot/rocker/blob/master/static/filesystem.tar?raw=true";
    let resp = reqwest::blocking::get(url).unwrap();
    let mut file = File::create(FILE_PATH).unwrap();
    file.write_all(&resp.bytes().unwrap()).unwrap();
}

fn extract_image_tar() {
    let mut ar = Archive::new(File::open(FILE_PATH).unwrap());
    ar.unpack(IMAGE_DIR).unwrap();
}

// For the time being, ./rocker will hold one kind of image at once.
// And everytime a new image is created, the old one is removed.
pub fn handle_image_command() {
    match std::fs::remove_dir_all(IMAGE_DIR) {
        Ok(_) => println!("Removing existing image.."),
        Err(_) => println!("Image didn't exist!"),
    };
    
    match std::fs::create_dir(IMAGE_DIR) {
        Ok(_) => println!("Creating new directory for it.."),
        Err(_) => println!("Something went wrong. Expect errors."),
    };

    download_image_tar();
    extract_image_tar();
}
