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

// Fragment shader for the Sun (star) - glowing yellow/orange with radial gradient
pub fn sun_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let position = fragment.position;
    
    // Calculate distance from center for radial effect
    let center_x = 400.0;
    let center_y = 300.0;
    let dx = position.x - center_x;
    let dy = position.y - center_y;
    let dist = (dx * dx + dy * dy).sqrt() / 200.0;
    
    // Core colors
    let core_color = Vector3::new(1.0, 1.0, 0.9);    // Bright white-yellow (core)
    let middle_color = Vector3::new(1.0, 0.8, 0.2);  // Yellow-orange (middle)
    let edge_color = Vector3::new(1.0, 0.5, 0.1);    // Orange-red (edge)
    
    // Pulsating effect
    let pulse = 0.9 + 0.1 * (fragment.depth * 5.0).sin();
    
    // Mix colors based on distance from center
    let color = if dist < 0.3 {
        // Core: mostly white-yellow
        Vector3::new(
            core_color.x * pulse,
            core_color.y * pulse,
            core_color.z * pulse,
        )
    } else if dist < 0.7 {
        // Middle: transition to yellow-orange
        let t = (dist - 0.3) / 0.4;
        Vector3::new(
            (core_color.x * (1.0 - t) + middle_color.x * t) * pulse,
            (core_color.y * (1.0 - t) + middle_color.y * t) * pulse,
            (core_color.z * (1.0 - t) + middle_color.z * t) * pulse,
        )
    } else {
        // Edge: orange-red
        let t = ((dist - 0.7) / 0.3).min(1.0);
        Vector3::new(
            (middle_color.x * (1.0 - t) + edge_color.x * t) * pulse,
            (middle_color.y * (1.0 - t) + edge_color.y * t) * pulse,
            (middle_color.z * (1.0 - t) + edge_color.z * t) * pulse,
        )
    };
    
    color
}

// Fragment shader for Earth (rocky planet) - blue oceans and green/brown continents
pub fn earth_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let position = fragment.position;
    
    // Create procedural continents using multiple noise octaves
    let noise1 = ((position.x * 0.02).sin() * (position.y * 0.02).cos()).abs();
    let noise2 = ((position.x * 0.05 + 100.0).cos() * (position.y * 0.05 + 100.0).sin()).abs();
    let noise3 = ((position.x * 0.01 - 50.0).sin() * (position.y * 0.01 - 50.0).cos()).abs();
    
    let combined_noise = noise1 * 0.5 + noise2 * 0.3 + noise3 * 0.2;
    
    // Define colors
    let deep_ocean = Vector3::new(0.0, 0.1, 0.3);    // Deep blue
    let shallow_ocean = Vector3::new(0.1, 0.3, 0.6); // Light blue
    let beach = Vector3::new(0.8, 0.7, 0.4);         // Sandy
    let grass = Vector3::new(0.2, 0.5, 0.2);         // Green
    let forest = Vector3::new(0.1, 0.3, 0.1);        // Dark green
    let mountain = Vector3::new(0.5, 0.4, 0.3);      // Brown
    let snow = Vector3::new(0.9, 0.9, 1.0);          // White-blue
    
    // Add polar ice caps based on y position
    let latitude = ((position.y - 300.0) / 300.0).abs();
    
    if latitude > 0.7 {
        // Polar regions - ice caps
        snow
    } else if combined_noise < 0.35 {
        // Deep ocean
        deep_ocean
    } else if combined_noise < 0.42 {
        // Shallow water
        shallow_ocean
    } else if combined_noise < 0.45 {
        // Beach/coast
        beach
    } else if combined_noise < 0.55 {
        // Grassland
        grass
    } else if combined_noise < 0.65 {
        // Forest
        forest
    } else if combined_noise < 0.75 {
        // Mountains
        mountain
    } else {
        // High mountains/snow peaks
        snow
    }
}

// Fragment shader for Gas Giant (Jupiter/Uranus) - prominent horizontal bands
pub fn gas_giant_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let position = fragment.position;
    
    // Create horizontal bands with turbulence
    let y_band = position.y * 0.02;
    let turbulence = (position.x * 0.03).sin() * 10.0;
    let band_pos = (y_band + turbulence * 0.1).sin();
    
    // Add swirls and details
    let detail = ((position.x * 0.05).cos() * (position.y * 0.03).sin()).abs();
    
    // Jupiter-style colors
    let color1 = Vector3::new(0.9, 0.7, 0.5);   // Cream/beige
    let color2 = Vector3::new(0.8, 0.5, 0.3);   // Light brown
    let color3 = Vector3::new(0.6, 0.3, 0.1);   // Dark brown
    let color4 = Vector3::new(0.95, 0.8, 0.6);  // Light cream
    
    // Create the Great Red Spot effect
    let spot_x = 450.0;
    let spot_y = 320.0;
    let dist_to_spot = ((position.x - spot_x).powi(2) + (position.y - spot_y).powi(2)).sqrt();
    
    if dist_to_spot < 40.0 {
        // Great Red Spot
        let spot_intensity = 1.0 - (dist_to_spot / 40.0);
        Vector3::new(
            0.7 + spot_intensity * 0.3,
            0.3 + spot_intensity * 0.2,
            0.2 + spot_intensity * 0.1,
        )
    } else if band_pos > 0.6 {
        // Light band
        let mix = detail * 0.3;
        Vector3::new(
            color1.x * (1.0 - mix) + color4.x * mix,
            color1.y * (1.0 - mix) + color4.y * mix,
            color1.z * (1.0 - mix) + color4.z * mix,
        )
    } else if band_pos > 0.2 {
        // Medium band
        let mix = detail * 0.4;
        Vector3::new(
            color2.x * (1.0 - mix) + color1.x * mix,
            color2.y * (1.0 - mix) + color1.y * mix,
            color2.z * (1.0 - mix) + color1.z * mix,
        )
    } else if band_pos > -0.2 {
        // Dark band
        let mix = detail * 0.5;
        Vector3::new(
            color3.x * (1.0 - mix) + color2.x * mix,
            color3.y * (1.0 - mix) + color2.y * mix,
            color3.z * (1.0 - mix) + color2.z * mix,
        )
    } else if band_pos > -0.6 {
        // Medium-light band
        let mix = detail * 0.4;
        Vector3::new(
            color2.x * (1.0 - mix) + color4.x * mix,
            color2.y * (1.0 - mix) + color4.y * mix,
            color2.z * (1.0 - mix) + color4.z * mix,
        )
    } else {
        // Very light band
        color4
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