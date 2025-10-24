use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;

// This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

// Fragment shader for the Sun (star) - glowing yellow/orange
pub fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let base_color = Vector3::new(1.0, 0.9, 0.2); // Bright yellow-orange
    let glow = Vector3::new(1.0, 0.8, 0.0); // Orange glow
    
    // Simple pulsating effect using depth
    let intensity = 0.8 + 0.2 * (fragment.depth * 10.0).sin();
    
    Vector3::new(
        (base_color.x * 0.7 + glow.x * 0.3) * intensity,
        (base_color.y * 0.7 + glow.y * 0.3) * intensity,
        (base_color.z * 0.7 + glow.z * 0.3) * intensity,
    )
}

// Fragment shader for Earth (rocky planet) - blue and green
pub fn earth_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    // Simple procedural coloring based on position
    let position = fragment.position;
    let noise = ((position.x * 0.1).sin() * (position.y * 0.1).cos()).abs();
    
    let ocean_color = Vector3::new(0.1, 0.3, 0.8); // Blue
    let land_color = Vector3::new(0.2, 0.6, 0.2);  // Green
    
    // Mix between ocean and land based on noise
    if noise > 0.5 {
        land_color
    } else {
        ocean_color
    }
}

// Fragment shader for Gas Giant (Jupiter/Uranus) - bands of color
pub fn gas_giant_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    // Create horizontal bands based on y position
    let position = fragment.position;
    let band = (position.y * 0.05).sin();
    
    let color1 = Vector3::new(0.8, 0.6, 0.4); // Light brown
    let color2 = Vector3::new(0.6, 0.4, 0.2); // Dark brown
    let color3 = Vector3::new(0.9, 0.7, 0.5); // Cream
    
    if band > 0.3 {
        color1
    } else if band > -0.3 {
        color2
    } else {
        color3
    }
}

// Simple default shader that uses vertex color or white
pub fn default_shader(fragment: &Fragment) -> Vector3 {
    // If fragment has color, use it, otherwise use white
    if fragment.color.x > 0.0 || fragment.color.y > 0.0 || fragment.color.z > 0.0 {
        fragment.color
    } else {
        Vector3::new(1.0, 1.0, 1.0) // White
    }
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
  let position_vec4 = Vector4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );

  // Apply the transformation by multiplying the model matrix with the vector
  let transformed_vec4 = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  // Perform perspective division to convert from homogeneous coordinates back to 3D Cartesian coordinates
  let transformed_position = if transformed_vec4.w != 0.0 {
      Vector3::new(
          transformed_vec4.x / transformed_vec4.w,
          transformed_vec4.y / transformed_vec4.w,
          transformed_vec4.z / transformed_vec4.w,
      )
  } else {
      // Avoid division by zero, though w should usually be 1 for model transformations
      Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z)
  };

  // Create a new Vertex with the transformed position
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: vertex.normal, // Note: Correct normal transformation is more complex
  }
}