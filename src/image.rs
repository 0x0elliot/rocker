use std::process::Command;

const IMAGE_DIR: &str = concat!(env!("HOME"), "/.rocker", "/images");

// Right now, it removes all images.
// This is a very experimental hack. Let it be for now.
fn remove_image() {
    let output = Command::new("rm")
        .arg("-rf")
        .arg(IMAGE_DIR)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn make_image_dir() {
    let output = Command::new("mkdir")
        .arg("-p")
        .arg(IMAGE_DIR)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}

fn docker_create_alpine() {
    let output = Command::new("docker")
        .arg("create")
        .arg("alpine")
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8(output.stdout).unwrap();
}

fn docker_export_alpine() {
    let image_hash = docker_create_alpine();
    let output = Command::new("docker")
        .arg("export")
        .arg(image_hash)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout);
}

fn image_to_tar() {
    let image = docker_export_alpine();
    let output = Command::new("tar")
        .arg("-C")
        .arg(IMAGE_DIR)
        .arg("-xvf")
        .arg(image)
        .output()
        .expect("failed to execute process");
    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    String::from_utf8_lossy(&output.stdout);
}



// For the time being, ./rocker will hold one kind of image at once.
// And everytime a new image is created, the old one is removed.
pub fn handle_image_command() {
    remove_image();
    make_image_dir();
}