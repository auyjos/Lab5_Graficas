use crate::vertex::Vertex;
use raylib::math::{Vector2, Vector3};
use tobj;

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, _materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // First pass: find bounds to normalize
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;
            let mut min_z = f32::MAX;
            let mut max_z = f32::MIN;

            for i in 0..num_vertices {
                let x = mesh.positions[i * 3];
                let y = mesh.positions[i * 3 + 1];
                let z = mesh.positions[i * 3 + 2];

                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
                min_z = min_z.min(z);
                max_z = max_z.max(z);
            }

            // Calculate center and scale
            let center = Vector3::new(
                (min_x + max_x) / 2.0,
                (min_y + max_y) / 2.0,
                (min_z + max_z) / 2.0,
            );

            let size_x = max_x - min_x;
            let size_y = max_y - min_y;
            let size_z = max_z - min_z;
            let max_size = size_x.max(size_y).max(size_z);

            // Scale to fit in [-1, 1]
            let scale = if max_size > 0.0 { 2.0 / max_size } else { 1.0 };

            // Second pass: normalize vertices
            for i in 0..num_vertices {
                let x = (mesh.positions[i * 3] - center.x) * scale;
                let y = -(mesh.positions[i * 3 + 1] - center.y) * scale; // Flip Y
                let z = (mesh.positions[i * 3 + 2] - center.z) * scale;
                let position = Vector3::new(x, y, z);

                let normal = if !mesh.normals.is_empty() {
                    let nx = mesh.normals[i * 3];
                    let ny = mesh.normals[i * 3 + 1];
                    let nz = mesh.normals[i * 3 + 2];
                    Vector3::new(nx, -ny, nz)
                } else {
                    Vector3::zero()
                };

                let tex_coords = if !mesh.texcoords.is_empty() {
                    let u = mesh.texcoords[i * 2];
                    let v = mesh.texcoords[i * 2 + 1];
                    Vector2::new(u, v)
                } else {
                    Vector2::zero()
                };

                vertices.push(Vertex::new(position, normal, tex_coords));
            }
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { vertices, indices })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();
        for &index in &self.indices {
            vertex_array.push(self.vertices[index as usize].clone());
        }
        vertex_array
    }
}
