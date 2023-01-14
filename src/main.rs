#![warn(clippy::all)]

mod camera;
mod light;
mod material;
mod math;
mod ray;
mod renderer;
mod scene;
mod shape;
mod threadpool;

use camera::Camera;
use math::vector::Vec3;

// For reading and opening files
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::sync::{Arc, RwLock};

use renderer::RenderSettings;

extern crate libc;
extern crate png;

pub const SCREEN_WIDTH: usize = 800;
pub const SCREEN_HEIGHT: usize = 600;

#[allow(dead_code)]
fn main() -> Result<(), std::io::Error> {
    let render_setting: RenderSettings = RenderSettings {
        screen_width: SCREEN_WIDTH,
        screen_height: SCREEN_HEIGHT,
    };

    // Scene is just a read only data object.
    let scene = Arc::new(RwLock::new(scene::create_scene()));

    // Create or overwrite file.
    let path = Path::new(r"other\images\progress.png");
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32);
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let image_data = renderer::render_scene(scene, &render_setting).unwrap();

    writer.write_image_data(&image_data).unwrap();

    println!("New image created: {}", path.display());

    Ok(())
}
