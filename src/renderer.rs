use super::material;
use super::math::vector::Vec3;
use super::ray::Ray;
use super::scene::Scene;
use super::shape;
use super::threadpool::ThreadPool;
extern crate rand;
use rand::prelude::*;

use std::ops::Deref;
use std::time::Instant;

pub const MAX_RAY_DEPTH: u16 = 50;
pub const RAYS_PER_PIXEL: u16 = 500;
pub const NUM_THREADS: usize = 16;
const BYTES_PIXEL: usize = 3;

pub struct RenderSettings {
    pub screen_width: usize,
    pub screen_height: usize,
}

pub fn render_scene(scene: Scene, render_setting: &RenderSettings) -> Result<Vec<u8>, String> {
    // Create jobs
    println!("Preparing..");

    let now = Instant::now();

    let mut pool = ThreadPool::new(NUM_THREADS, scene);

    // Create buffer on heap.
    let buffer_size: usize =
        render_setting.screen_width * render_setting.screen_height * BYTES_PIXEL;

    let mut image = {
        let mut v: Vec<u8> = Vec::with_capacity(buffer_size);
        unsafe {
            v.set_len(buffer_size);
        };

        v
    };

    println!(
        "Start rendering..
    Size {}x{}
    Number of threads: {}
    Max Ray Depth: {}
    Ray Per Pixel {}",
        render_setting.screen_width,
        render_setting.screen_height,
        NUM_THREADS,
        MAX_RAY_DEPTH,
        RAYS_PER_PIXEL
    );

    let mut index = 0;
    for y in 0..super::SCREEN_HEIGHT {
        for x in 0..super::SCREEN_WIDTH {
            pool.schedule((x, y, index), &render_pixel_job);
            index = index + BYTES_PIXEL;
        }
    }

    pool.wait_all();

    // Combine results into image.
    for worker in pool.workers.iter_mut() {
        let lock = worker.results.lock().unwrap();
        //let ref_list = worker.results.into_inner().unwrap();
        for (index, (r, g, b)) in lock.deref().clone() {
            image[index] = r;
            image[index + 1] = g;
            image[index + 2] = b;
        }
    }

    println!(
        "Finished rendering: {} Seconds",
        now.elapsed().as_secs_f32()
    );

    Ok(image)
}

fn render_pixel_job(scene: &Scene, coordinate: (usize, usize, usize)) -> (u8, u8, u8) {
    let mut rng = rand::thread_rng();

    let mut pixel_color = Vec3::zero();
    for _n_rp in 0..RAYS_PER_PIXEL {
        let rand_coord: (f64, f64) = rng.gen();

        let mut r = scene.camera.generate_ray(
            coordinate.0 as f64 + rand_coord.0,
            coordinate.1 as f64 + rand_coord.1,
        );
        pixel_color += raytrace(&scene, &mut r, 0);
    }

    to_color(pixel_color, RAYS_PER_PIXEL)
}

fn raytrace(scene: &Scene, ray: &mut Ray, depth: u16) -> Vec3 {
    if depth >= MAX_RAY_DEPTH {
        return Vec3(0.0, 0.0, 0.0);
    }

    // TODO: BVH for intersecting?
    for it in scene.objects.iter() {
        //it.intersect(ray, 0.001);
        shape::intersect(it, ray, 0.001);
    }

    if let Some(hit) = &ray.is_intersected {
        if let Some((attenuation, mut scattered_ray)) = material::scatter(&hit.material, ray) {
            return attenuation * raytrace(scene, &mut scattered_ray, depth + 1);
        }

        return Vec3(0.0, 0.0, 0.0);
    }

    //sky
    let t = 0.5 * (ray.direction.1 + 1.0);
    Vec3::fill(1.0) * (1.0 - t) + (Vec3(0.5, 0.7, 1.0) * t)
}

// Returning rgb u8
fn to_color(vec: Vec3, samples: u16) -> (u8, u8, u8) {
    let scale = 1.0 / samples as f64;

    let _r = (scale * vec.0).sqrt();
    let _g = (scale * vec.1).sqrt();
    let _b = (scale * vec.2).sqrt();

    let _r = f64::clamp(_r, 0.0, 0.999999) * 256.0;
    let _g = f64::clamp(_g, 0.0, 0.999999) * 256.0;
    let _b = f64::clamp(_b, 0.0, 0.999999) * 256.0;

    (_r as u8, _g as u8, _b as u8)
}
