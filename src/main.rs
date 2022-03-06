use std::io;

fn main() {}

struct Material {
    name: String,
    resistivity: f32,
    conductivity: f32,
    series: i32,
    density: f32,
    percentage: f32,
}

struct Wire {
    diameter: f32,
    material: Material,
    strength: f32,
    unit_wieght: f32,
}

impl Wire {
    fn area(&self) -> f32 {
        self.diameter.powf(2.) / 4.
    }
}
