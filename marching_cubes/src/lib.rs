pub mod lookup_tables;

use lookup_tables::{EDGE_CONNECTIONS, EDGE_TABLE, TRI_TABLE};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Metaball {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    influence: f32,
}

#[wasm_bindgen]
impl Metaball {
    #[wasm_bindgen(constructor)]
    pub fn new(x: f32, y: f32, z: f32, radius: f32, influence: f32) -> Metaball {
        Metaball {
            x,
            y,
            z,
            radius,
            influence,
        }
    }
}

#[wasm_bindgen]
pub struct GridData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    normals: Vec<f32>,
}

#[wasm_bindgen]
impl GridData {
    #[wasm_bindgen(getter)]
    pub fn vertices(&self) -> Vec<f32> {
        self.vertices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn indices(&self) -> Vec<u32> {
        self.indices.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn normals(&self) -> Vec<f32> {
        self.normals.clone()
    }
}

fn scalar_field(x: f32, y: f32, z: f32, metaballs: &Box<[Metaball]>) -> f32 {
    metaballs.iter().fold(0.0, |sum, ball| {
        let dx = x - ball.x;
        let dy = y - ball.y;
        let dz = z - ball.z;
        let distance_squared = dx * dx + dy * dy + dz * dz;

        // Hermite cubic interpolation for blending
        let influence = ball.influence / (distance_squared + ball.radius);
        let normalized_distance = (distance_squared / ball.radius).sqrt().min(1.0); // Clamp to [0, 1]
        let smooth_factor = 1.0
            - normalized_distance
                * normalized_distance
                * normalized_distance
                * (normalized_distance * (normalized_distance * 6.0 - 15.0) + 10.0);

        sum + influence * smooth_factor
    })
}

#[wasm_bindgen]
pub fn marching_cubes(resolution: i32, metaballs: Box<[Metaball]>, threshold: f32) -> GridData {
    let grid_size = 1.0 / resolution as f32;
    let half_res = resolution / 2;

    let mut vertices: Vec<f32> = Vec::new();
    let mut indices = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    let compute_gradient = |x: f32, y: f32, z: f32| -> (f32, f32, f32) {
        let delta = 0.001; // Step size for finite differences

        let dx =
            scalar_field(x + delta, y, z, &metaballs) - scalar_field(x - delta, y, z, &metaballs);
        let dy =
            scalar_field(x, y + delta, z, &metaballs) - scalar_field(x, y - delta, z, &metaballs);
        let dz =
            scalar_field(x, y, z + delta, &metaballs) - scalar_field(x, y, z - delta, &metaballs);

        let magnitude = (dx * dx + dy * dy + dz * dz).sqrt();
        (-dx / magnitude, -dy / magnitude, -dz / magnitude) // Flip direction
    };

    let mut vertex_count = 0;

    for x in -half_res..half_res {
        for y in -half_res..half_res {
            for z in -half_res..half_res {
                // Define the 8 corners of the cube
                let corners: [(f32, f32, f32); 8] = [
                    (
                        x as f32 * grid_size,
                        y as f32 * grid_size,
                        z as f32 * grid_size,
                    ),
                    (
                        (x + 1) as f32 * grid_size,
                        y as f32 * grid_size,
                        z as f32 * grid_size,
                    ),
                    (
                        (x + 1) as f32 * grid_size,
                        (y + 1) as f32 * grid_size,
                        z as f32 * grid_size,
                    ),
                    (
                        x as f32 * grid_size,
                        (y + 1) as f32 * grid_size,
                        z as f32 * grid_size,
                    ),
                    (
                        x as f32 * grid_size,
                        y as f32 * grid_size,
                        (z + 1) as f32 * grid_size,
                    ),
                    (
                        (x + 1) as f32 * grid_size,
                        y as f32 * grid_size,
                        (z + 1) as f32 * grid_size,
                    ),
                    (
                        (x + 1) as f32 * grid_size,
                        (y + 1) as f32 * grid_size,
                        (z + 1) as f32 * grid_size,
                    ),
                    (
                        x as f32 * grid_size,
                        (y + 1) as f32 * grid_size,
                        (z + 1) as f32 * grid_size,
                    ),
                ];

                // Compute scalar field values at corners
                let mut corner_values = [0.0; 8];
                for i in 0..8 {
                    corner_values[i] =
                        scalar_field(corners[i].0, corners[i].1, corners[i].2, &metaballs);
                }

                // Determine cube index using the scalar field values
                let mut cube_index = 0;
                for (i, &value) in corner_values.iter().enumerate() {
                    if value >= threshold {
                        cube_index |= 1 << i;
                    }
                }

                // Skip if the cube is entirely inside or outside the surface
                if EDGE_TABLE[cube_index] == 0 {
                    continue;
                }

                // Interpolate vertices along edges
                let mut edge_vertices = [None; 12];

                for i in 0..12 {
                    if (EDGE_TABLE[cube_index] & (1 << i)) != 0 {
                        let (v1, v2) = EDGE_CONNECTIONS[i];
                        let p1 = corners[v1];
                        let p2 = corners[v2];
                        let val1 = corner_values[v1];
                        let val2 = corner_values[v2];

                        let t = (threshold - val1) / (val2 - val1);
                        let interpolated = (
                            p1.0 + t * (p2.0 - p1.0),
                            p1.1 + t * (p2.1 - p1.1),
                            p1.2 + t * (p2.2 - p1.2),
                        );

                        edge_vertices[i] = Some(interpolated);
                    }
                }

                let start_index = cube_index * 16;
                let tri_indices = &TRI_TABLE[start_index..start_index + 16];

                for tri in tri_indices.chunks(3) {
                    if tri[0] == -1 {
                        break;
                    }

                    for &edge_index in tri {
                        if let Some(vertex) = edge_vertices[edge_index as usize] {
                            vertices.extend_from_slice(&[vertex.0, vertex.1, vertex.2]);

                            let (nx, ny, nz) = compute_gradient(vertex.0, vertex.1, vertex.2);
                            let magnitude = (nx * nx + ny * ny + nz * nz).sqrt();
                            normals.extend_from_slice(&[
                                nx / magnitude,
                                ny / magnitude,
                                nz / magnitude,
                            ]);
                        }
                    }

                    indices.extend_from_slice(&[vertex_count + 2, vertex_count + 1, vertex_count]);
                    vertex_count += 3;
                }
            }
        }
    }

    GridData {
        vertices,
        indices,
        normals,
    }
}

#[wasm_bindgen]
pub fn visualize_sdf(resolution: f32, metaballs: Box<[Metaball]>, threshold: f32) -> GridData {
    let mut vertices = Vec::new();
    let indices = Vec::new();

    // render a small quad with a vertex color based on the sdf value
    let grid_size = 1.0 / resolution as f32;
    let half_res = (resolution / 2.0) as i32;

    for x in -half_res..half_res {
        for y in -half_res..half_res {
            for z in -half_res..half_res {
                let sdf = scalar_field(
                    x as f32 * grid_size,
                    y as f32 * grid_size,
                    z as f32 * grid_size,
                    &metaballs,
                );

                if sdf < threshold {
                    continue;
                }

                vertices.extend_from_slice(&[
                    x as f32 * grid_size,
                    y as f32 * grid_size,
                    z as f32 * grid_size,
                ]);
            }
        }
    }

    GridData {
        vertices,
        indices,
        normals: Vec::new(),
    }
}
