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

extern crate png;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

#[allow(dead_code)]
fn main() -> Result<(), std::io::Error> {
    let scene = scene::create_scene();

    // Create or overwrite file.
    let path = Path::new(r"/other/images/new2.png");
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SCREEN_WIDTH, SCREEN_HEIGHT);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    let mut stream_writer = writer.stream_writer();

    let data = [255, 0, 0, 255, 0, 0, 0, 255];
    stream_writer.write(data).unwrap();
    stream_writer.finish();
    //renderer::render_scene(&scene, &mut canvas)?;

    Ok(())
}
