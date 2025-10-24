use raylib::prelude::*;
use crate::vertex::Vertex;

/// Generate a torus geometry for planet rings
#[allow(dead_code)]
pub fn generate_torus_ring(major_radius: f32, minor_radius: f32, major_segments: usize, minor_segments: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    let two_pi = std::f32::consts::PI * 2.0;
    
    for i in 0..=major_segments {
        let u = (i as f32) / (major_segments as f32) * two_pi;
        let (sin_u, cos_u) = u.sin_cos();
        
        for j in 0..=minor_segments {
            let v = (j as f32) / (minor_segments as f32) * two_pi;
            let (sin_v, cos_v) = v.sin_cos();
            
            // Torus parametric equations
            let x = (major_radius + minor_radius * cos_v) * cos_u;
            let y = minor_radius * sin_v;
            let z = (major_radius + minor_radius * cos_v) * sin_u;
            
            let position = Vector3::new(x, y, z);
            
            // Normal pointing outward from torus surface
            let normal = Vector3::new(
                cos_v * cos_u,
                sin_v,
                cos_v * sin_u,
            );
            
            vertices.push(Vertex::new(
                position,
                normal,
                Vector2::new(u / two_pi, v / two_pi),
            ));
        }
    }
    
    vertices
}

/// Generate a simple disk ring (flat)
pub fn generate_flat_ring(inner_radius: f32, outer_radius: f32, segments: usize) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    let two_pi = std::f32::consts::PI * 2.0;
    
    for i in 0..segments {
        let angle1 = (i as f32) / (segments as f32) * two_pi;
        let angle2 = ((i + 1) as f32) / (segments as f32) * two_pi;
        
        let (sin1, cos1) = angle1.sin_cos();
        let (sin2, cos2) = angle2.sin_cos();
        
        // Inner point 1
        vertices.push(Vertex::new(
            Vector3::new(inner_radius * cos1, 0.0, inner_radius * sin1),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        
        // Outer point 1
        vertices.push(Vertex::new(
            Vector3::new(outer_radius * cos1, 0.0, outer_radius * sin1),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(1.0, 0.0),
        ));
        
        // Outer point 2
        vertices.push(Vertex::new(
            Vector3::new(outer_radius * cos2, 0.0, outer_radius * sin2),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(1.0, 1.0),
        ));
        
        // Inner point 1 (again for second triangle)
        vertices.push(Vertex::new(
            Vector3::new(inner_radius * cos1, 0.0, inner_radius * sin1),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 0.0),
        ));
        
        // Outer point 2
        vertices.push(Vertex::new(
            Vector3::new(outer_radius * cos2, 0.0, outer_radius * sin2),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(1.0, 1.0),
        ));
        
        // Inner point 2
        vertices.push(Vertex::new(
            Vector3::new(inner_radius * cos2, 0.0, inner_radius * sin2),
            Vector3::new(0.0, 1.0, 0.0),
            Vector2::new(0.0, 1.0),
        ));
    }
    
    vertices
}
