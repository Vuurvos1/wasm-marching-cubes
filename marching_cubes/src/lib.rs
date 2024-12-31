use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn marching_cubes(size: f32) -> Vec<f32> {
    let half_size = size / 2.0;
    let vertices = vec![
        -half_size, -half_size, half_size, // v0
        half_size, -half_size, half_size, // v1
        half_size, half_size, half_size, // v2
        half_size, half_size, half_size, // v3
        -half_size, half_size, half_size, // v4
        -half_size, -half_size, half_size, // v5
    ];

    vertices
}
