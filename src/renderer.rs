use super::math;
use super::math::vector::Vec3;
use super::ray::Ray;
use super::scene::Scene;

extern crate rand;
use rand::prelude::*;

use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Instant;

pub const MAX_RAY_DEPTH: u16 = 50;
pub const RAYS_PER_PIXEL: u16 = 16;
pub const NUM_THREADS: u8 = 4;
const BYTES_PIXEL: usize = 3;

// struct render_job<'a> {
//     screen_position: (u32, u32),
//     chunk: &'a mut [u8],
// }

pub fn render_scene<'a>(
    scene: Arc<RwLock<Scene>>,
    canvas: &'static mut [u8],
) -> Result<(), String> {
    println!("Start rendering..");
    let now = Instant::now();

    let mut threads = vec![];

    // threads.push(thread::spawn(render_to_canvas
    //
    // ));

    const TOTAL_PIXELS: usize = super::SCREEN_WIDTH * super::SCREEN_HEIGHT;

    const NUM_CHUNKS: usize = 4;
    const CHUNK_SIZE_BYTES: usize = (TOTAL_PIXELS / NUM_CHUNKS) * BYTES_PIXEL;
    // Create list of chunks
    let chunks = canvas.chunks_mut(CHUNK_SIZE_BYTES);

    // Need to know the xy position in order to draw it
    for (i, chunk) in chunks.enumerate() {
        let arc_scene = scene.clone();
        let bytes_chunk = i * CHUNK_SIZE_BYTES;
        let x = bytes_chunk % (super::SCREEN_WIDTH * BYTES_PIXEL); // remainder
        let y = bytes_chunk / (super::SCREEN_WIDTH * BYTES_PIXEL);
        println!("Chunk[{}]: x:{}, y:{}", i, x, y);
        threads.push(thread::spawn(move || {
            render_to_canvas(&arc_scene, chunk, (x, y))
        }));
    }

    // wait for childeren to finish
    for thread in threads {
        let _ = thread.join().unwrap();
    }
    println!(
        "Finished rendering: {} Milliseconds",
        now.elapsed().as_millis()
    );
    Ok(())
}

fn render_to_canvas<'a>(
    scene: &Arc<RwLock<Scene>>,
    canvas: &'a mut [u8],
    screen_coords: (usize, usize),
) -> Result<(), String> {
    let mut rng = rand::thread_rng();

    //let mut index: usize = 0;
    let (mut x, mut y) = screen_coords;

    for pixel in (0..canvas.len()).step_by(BYTES_PIXEL) {
        let mut pixel_color = Vec3::zero();
        for n_rp in 0..RAYS_PER_PIXEL {
            let rand_coord: (f64, f64) = rng.gen();
            let mut r = scene
                .read()
                .unwrap()
                .camera
                .generate_ray(x as f64 + rand_coord.0, y as f64 + rand_coord.1);
            pixel_color += raytrace(scene, &mut r, 0);
        }
        let (r, g, b) = to_color(pixel_color, RAYS_PER_PIXEL);

        canvas[pixel] = r;
        canvas[pixel + 1] = g;
        canvas[pixel + 2] = b;

        x = x + 1;
        if x >= super::SCREEN_WIDTH {
            x = 0;
            y = y + 1;
        }
    }

    Ok(())
}

/*fn render_to_canvas(scene: &Scene, canvas: &mut [u8]) -> Result<(), String> {
    let mut rng = rand::thread_rng();

    let mut index: usize = 0;
    for y in 0..super::SCREEN_HEIGHT {
        for x in 0..super::SCREEN_WIDTH {
            let mut pixel_color = Vec3::zero();
            for n_rp in 0..RAYS_PER_PIXEL {
                let rand_coord: (f64, f64) = rng.gen();
                let mut r = scene
                    .camera
                    .generate_ray(x as f64 + rand_coord.0, y as f64 + rand_coord.1);
                pixel_color += raytrace(&scene, &mut r, 0);
            }
            let (r, g, b) = to_color(pixel_color, RAYS_PER_PIXEL);
            canvas[index] = r;
            canvas[index + 1] = g;
            canvas[index + 2] = b;
            index = index + 3;
        }
    }
    Ok(())
}*/

fn raytrace(scene: &Arc<RwLock<Scene>>, ray: &mut Ray, depth: u16) -> Vec3 {
    if depth >= MAX_RAY_DEPTH {
        return Vec3(0.0, 0.0, 0.0);
    }

    // TODO: BVH for intersecting?
    for it in scene.read().unwrap().objects.iter() {
        it.intersect(ray, 0.001);
    }

    if let Some(hit) = &ray.is_intersected {
        if let Some((attenuation, mut scattered_ray)) = hit.material.scatter(ray) {
            return attenuation * raytrace(scene, &mut scattered_ray, depth + 1);
        }

        //let target = hit.position + Vec3::rand_in_hemispere(hit.normal);
        return Vec3(0.0, 0.0, 0.0);
        //return (hit.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
    }

    let t = 0.5 * (ray.direction.1 + 1.0);
    Vec3::fill(1.0) * (1.0 - t) + (Vec3(0.5, 0.7, 1.0) * t)
}

// Returning rgb u8
fn to_color(vec: Vec3, samples: u16) -> (u8, u8, u8) {
    let scale = 1.0 / samples as f64;

    let _r = (scale * vec.0).sqrt();
    let _g = (scale * vec.1).sqrt();
    let _b = (scale * vec.2).sqrt();

    let _r = f64::clamp(_r, 0.0, 0.9999) * 256.0;
    let _r = math::clamp(_r, 0.0, 0.9999) * 256.0;
    let _g = math::clamp(_g, 0.0, 0.9999) * 256.0;
    let _b = math::clamp(_b, 0.0, 0.9999) * 256.0;

    (_r as u8, _g as u8, _b as u8)
}
