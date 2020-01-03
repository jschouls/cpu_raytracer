use sdl2::pixels;

pub struct Material {
    pub color: pixels::Color,
    pub reflection: f32,
    pub refraction: f32,
}
