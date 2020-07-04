#![warn(clippy::all)]

mod camera;
mod light;
mod material;
mod math;
mod ray;
mod renderer;
mod scene;
mod shape;

use camera::Camera;
use math::vector::Vec3;

// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

extern crate libc;
extern crate png;

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;
//pub const THREADS: u8 = 4;

#[allow(dead_code)]
fn main() -> Result<(), std::io::Error> {
    let scene = scene::create_scene();

    // Create or overwrite file.
    let path = Path::new(r"other\images\progress.png");
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    // Create buffer on heap.
    const BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT * 3;
    let mut image = {
        let mut v = Vec::with_capacity(BUFFER_SIZE);
        unsafe {
            v.set_len(BUFFER_SIZE);
        };

        // We dont need to set to a default value, because we going to overwrite it any way
        v.into_boxed_slice()
    };

    renderer::render_scene(&scene, &mut image).unwrap();

    writer.write_image_data(&image).unwrap();
    println!("New image created: {}", path.display());
    Ok(())
}
