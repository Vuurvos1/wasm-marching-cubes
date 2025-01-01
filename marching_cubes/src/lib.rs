pub mod lookup_tables;

use lookup_tables::{EDGE_CONNECTIONS, EDGE_TABLE, TRI_TABLE};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GridData {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    colors: Vec<f32>,
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
    pub fn colors(&self) -> Vec<f32> {
        self.colors.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn normals(&self) -> Vec<f32> {
        self.normals.clone()
    }
}

#[wasm_bindgen]
pub fn marching_cubes(size: f32) -> GridData {
    let resolution = 32;
    let grid_size = 1.0 / resolution as f32;
    let half_res = resolution / 2;

    let mut vertices: Vec<f32> = Vec::new();
    let mut indices = Vec::new();
    let mut colors = Vec::new();
    let mut normals: Vec<f32> = Vec::new();

    // Function to compute scalar field value (sphere signed distance)
    let sphere_sdf = |x: f32, y: f32, z: f32| -> f32 {
        let radius = 0.5;
        let center = (0.0, 0.0, 0.0);
        let dx = x - center.0;
        let dy = y - center.1;
        let dz = z - center.2;
        (dx * dx + dy * dy + dz * dz).sqrt() - radius
    };

    let compute_gradient = |x: f32, y: f32, z: f32| -> (f32, f32, f32) {
        let delta = 0.001; // Small step for gradient calculation
        let dx = sphere_sdf(x + delta, y, z) - sphere_sdf(x - delta, y, z);
        let dy = sphere_sdf(x, y + delta, z) - sphere_sdf(x, y - delta, z);
        let dz = sphere_sdf(x, y, z + delta) - sphere_sdf(x, y, z - delta);
        (dx, dy, dz)
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
                    corner_values[i] = sphere_sdf(corners[i].0, corners[i].1, corners[i].2);
                }

                // Determine cube index using the scalar field values
                let mut cube_index = 0;
                for (i, &value) in corner_values.iter().enumerate() {
                    if value < 0.0 {
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

                        let t = val1 / (val1 - val2); // Linear interpolation factor
                        let interpolated = (
                            p1.0 + t * (p2.0 - p1.0),
                            p1.1 + t * (p2.1 - p1.1),
                            p1.2 + t * (p2.2 - p1.2),
                        );

                        edge_vertices[i] = Some(interpolated);
                    }
                }

                // Generate triangles using TRI_TABLE
                let start_index = cube_index * 16;
                let tri_indices = &TRI_TABLE[start_index..start_index + 16];

                for tri in tri_indices.chunks(3) {
                    if tri[0] == -1 {
                        break; // End of triangle data for this cube configuration
                    }

                    for &edge_index in tri {
                        if let Some(vertex) = edge_vertices[edge_index as usize] {
                            vertices.extend_from_slice(&[vertex.0, vertex.1, vertex.2]);

                            // Calculate color based on distance from the center
                            let dist =
                                (vertex.0.powi(2) + vertex.1.powi(2) + vertex.2.powi(2)).sqrt();
                            colors.push(0.5 - dist);

                            // Calculate and normalize the gradient as the normal
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
        colors,
        normals,
    }
}
