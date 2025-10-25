use crate::vertex::Vertex;
use crate::texture::Texture;
use raylib::math::{Vector2, Vector3};
use tobj;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct Material {
    pub name: String,
    pub ambient: Vector3,
    pub diffuse: Vector3,
    pub specular: Vector3,
    pub shininess: f32,
    pub texture_path: Option<String>,
}

pub struct Obj {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub materials: Vec<Material>,
    pub mesh_materials: Vec<Option<usize>>, // Material index for each mesh
    pub texture: Option<Texture>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Self, tobj::LoadError> {
        let (models, materials_result) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;

        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let mut mesh_materials = Vec::new();
        let mut materials = Vec::new();

        // Process materials if available
        if let Ok(mats) = materials_result {
            for mat in mats {
                let ambient = if let Some(amb) = mat.ambient {
                    Vector3::new(amb[0], amb[1], amb[2])
                } else {
                    Vector3::new(0.2, 0.2, 0.2)
                };

                let diffuse = if let Some(diff) = mat.diffuse {
                    Vector3::new(diff[0], diff[1], diff[2])
                } else {
                    Vector3::new(1.0, 1.0, 1.0)
                };

                let specular = if let Some(spec) = mat.specular {
                    Vector3::new(spec[0], spec[1], spec[2])
                } else {
                    Vector3::new(1.0, 1.0, 1.0)
                };

                let shininess = mat.shininess.unwrap_or(32.0);
                
                // Get texture path if available
                let texture_path = mat.diffuse_texture.clone();

                materials.push(Material {
                    name: mat.name,
                    ambient,
                    diffuse,
                    specular,
                    shininess,
                    texture_path,
                });
            }
        }
        
        // Try to load texture from the first material that has one
        let mut texture = None;
        let base_path = std::path::Path::new(path).parent().unwrap_or(std::path::Path::new("."));
        
        for mat in &materials {
            if let Some(tex_path) = &mat.texture_path {
                let full_path = base_path.join(tex_path);
                if let Ok(tex) = Texture::load(full_path.to_str().unwrap_or("")) {
                    println!("✓ Loaded texture: {:?}", full_path);
                    texture = Some(tex);
                    break;
                } else {
                    eprintln!("✗ Failed to load texture: {:?}", full_path);
                }
            }
        }

        for model in models {
            let mesh = &model.mesh;
            let num_vertices = mesh.positions.len() / 3;

            // Store material index for this mesh
            let material_idx = mesh.material_id;
            mesh_materials.push(material_idx);

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

                // Get material color if available
                // If diffuse is pure white (common for textured models), use ambient instead
                let mut material_color = if let Some(mat_idx) = material_idx {
                    if mat_idx < materials.len() {
                        let mat = &materials[mat_idx];
                        // Check if diffuse is pure white (1.0, 1.0, 1.0)
                        if (mat.diffuse.x - 1.0).abs() < 0.01 && 
                           (mat.diffuse.y - 1.0).abs() < 0.01 && 
                           (mat.diffuse.z - 1.0).abs() < 0.01 {
                            // Use ambient color instead
                            mat.ambient
                        } else {
                            mat.diffuse
                        }
                    } else {
                        Vector3::new(1.0, 1.0, 1.0)
                    }
                } else {
                    Vector3::new(1.0, 1.0, 1.0)
                };
                
                // If texture is available, sample it and blend with material color
                if let Some(ref tex) = texture {
                    let tex_color = tex.sample_bilinear(tex_coords.x, tex_coords.y);
                    // Blend texture 70% with material 30% for nice procedural/texture mix
                    material_color = Vector3::new(
                        tex_color.x * 0.7 + material_color.x * 0.3,
                        tex_color.y * 0.7 + material_color.y * 0.3,
                        tex_color.z * 0.7 + material_color.z * 0.3,
                    );
                }

                let mut vertex = Vertex::new(position, normal, tex_coords);
                vertex.color = material_color;
                vertices.push(vertex);
            }
            indices.extend_from_slice(&mesh.indices);
        }

        Ok(Obj { 
            vertices, 
            indices,
            materials,
            mesh_materials,
            texture,
        })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();
        for &index in &self.indices {
            vertex_array.push(self.vertices[index as usize].clone());
        }
        vertex_array
    }
    
    pub fn get_texture(&self) -> &Option<Texture> {
        &self.texture
    }

    pub fn get_materials(&self) -> &Vec<Material> {
        &self.materials
    }
}
