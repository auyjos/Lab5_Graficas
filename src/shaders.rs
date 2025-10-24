use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::Uniforms;

// This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
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
  let transformed_position_3d = if transformed_vec4.w != 0.0 {
      Vector3::new(
          transformed_vec4.x / transformed_vec4.w,
          transformed_vec4.y / transformed_vec4.w,
          transformed_vec4.z / transformed_vec4.w,
      )
  } else {
      // Avoid division by zero, though w should usually be 1 for model transformations
      Vector3::new(transformed_vec4.x, transformed_vec4.y, transformed_vec4.z)
  };

  // Simple isometric projection with subtle Z effect
  // The Z position slightly affects Y and scale but doesn't distort too much
  let z_factor = transformed_position_3d.z * 0.02;
  
  let transformed_position = Vector3::new(
      transformed_position_3d.x,
      transformed_position_3d.y + z_factor * 5.0, // Slight vertical shift for depth
      transformed_position_3d.z,
  );

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

// ==================== FRAGMENT SHADERS ====================

/// Simple hash function for noise generation
fn hash(x: f32) -> f32 {
    let h = (x * 43758.5453).sin();
    h - h.floor()
}

/// 2D noise function
fn noise(p: Vector2) -> f32 {
    let i = Vector2::new(p.x.floor(), p.y.floor());
    let f = Vector2::new(p.x - i.x, p.y - i.y);
    
    // Four corners
    let a = hash(i.x + i.y * 57.0);
    let b = hash(i.x + 1.0 + i.y * 57.0);
    let c = hash(i.x + (i.y + 1.0) * 57.0);
    let d = hash(i.x + 1.0 + (i.y + 1.0) * 57.0);
    
    // Smooth interpolation
    let u = f.x * f.x * (3.0 - 2.0 * f.x);
    let v = f.y * f.y * (3.0 - 2.0 * f.y);
    
    let result = a * (1.0 - u) * (1.0 - v) +
                 b * u * (1.0 - v) +
                 c * (1.0 - u) * v +
                 d * u * v;
    result
}

/// Fractal Brownian Motion (FBM) for natural patterns
fn fbm(p: Vector2, octaves: i32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    let mut max_value = 0.0;
    
    for _ in 0..octaves {
        value += amplitude * noise(p * frequency);
        max_value += amplitude;
        amplitude *= 0.5;
        frequency *= 2.0;
    }
    
    value / max_value
}

/// Mix/Lerp two colors
fn mix_color(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    Vector3::new(
        a.x * (1.0 - t) + b.x * t,
        a.y * (1.0 - t) + b.y * t,
        a.z * (1.0 - t) + b.z * t,
    )
}

/// Smooth step function
fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// SUN SHADER - Golden/Orange with turbulent patterns
fn sun_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // UV coordinates from position
    let pos = vertex.transformed_position;
    let len = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    if len < 0.001 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let norm = Vector3::new(pos.x / len, pos.y / len, pos.z / len);
    let u = (norm.x.atan2(norm.z) / std::f32::consts::PI + 1.0) * 0.5;
    let v = (norm.y).asin() / std::f32::consts::PI + 0.5;
    
    let uv = Vector2::new(u, v);
    
    // Layer 1: Base sun color (golden yellow)
    let base = Vector3::new(1.0, 0.9, 0.0);
    
    // Layer 2: Add surface turbulence
    let turbulence = fbm(uv * 5.0 + time * 0.3, 3);
    let surface = mix_color(base, Vector3::new(1.0, 0.5, 0.0), turbulence * 0.5);
    
    // Layer 3: Add corona effect (outer glow)
    let corona_pattern = fbm(uv * 8.0 + time * 0.5, 2);
    let corona = mix_color(surface, Vector3::new(1.0, 0.8, 0.0), corona_pattern * 0.3);
    
    // Layer 4: Add bright spots (solar flares)
    let flares = (time.sin() * (uv.x * 20.0).sin() * (uv.y * 15.0).sin()).abs();
    let result = mix_color(corona, Vector3::new(1.0, 1.0, 0.5), flares * 0.2);
    
    result
}

/// EARTH-LIKE PLANET - Blue/Green with continents
fn earth_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // UV coordinates from position
    let pos = vertex.transformed_position;
    let len = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    if len < 0.001 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let norm = Vector3::new(pos.x / len, pos.y / len, pos.z / len);
    let u = (norm.x.atan2(norm.z) / std::f32::consts::PI + 1.0) * 0.5;
    let v = (norm.y).asin() / std::f32::consts::PI + 0.5;
    
    let uv = Vector2::new(u, v);
    
    // Layer 1: Ocean color (deep blue)
    let ocean = Vector3::new(0.0, 0.2, 0.6);
    
    // Layer 2: Continents (landmasses with noise)
    let land_noise = fbm(uv * 4.0, 3);
    let land_color = Vector3::new(0.2, 0.6, 0.2); // Green
    let with_land = mix_color(ocean, land_color, smoothstep(0.4, 0.7, land_noise));
    
    // Layer 3: Cloud patterns
    let cloud_noise = fbm(uv * 6.0 + time * 0.1, 2);
    let clouds = Vector3::new(1.0, 1.0, 1.0);
    let with_clouds = mix_color(with_land, clouds, smoothstep(0.6, 0.8, cloud_noise) * 0.6);
    
    // Layer 4: Polar ice caps
    let ice_latitude = smoothstep(0.8, 1.0, v) + smoothstep(0.2, 0.0, v);
    let ice_color = Vector3::new(0.9, 0.95, 1.0);
    let result = mix_color(with_clouds, ice_color, ice_latitude * 0.7);
    
    result
}

/// GAS GIANT - Orange/Red with bands and storms
fn gas_giant_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    // UV coordinates from position
    let pos = vertex.transformed_position;
    let len = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    if len < 0.001 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let norm = Vector3::new(pos.x / len, pos.y / len, pos.z / len);
    let u = (norm.x.atan2(norm.z) / std::f32::consts::PI + 1.0) * 0.5;
    let v = (norm.y).asin() / std::f32::consts::PI + 0.5;
    
    let uv = Vector2::new(u, v);
    
    // Layer 1: Base color - orangish/reddish
    let base = Vector3::new(0.8, 0.6, 0.3);
    
    // Layer 2: Band patterns (striped effect)
    let bands = ((v * 10.0).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_color = Vector3::new(0.7, 0.5, 0.2);
    let with_bands = mix_color(base, band_color, bands * 0.6);
    
    // Layer 3: Swirling storm patterns
    let storm = fbm(uv * 3.0 + time * 0.2, 3);
    let storm_color = Vector3::new(0.6, 0.3, 0.1); // Darker reddish-brown
    let with_storm = mix_color(with_bands, storm_color, storm * 0.5);
    
    // Layer 4: Great Red Spot (large storm)
    let spot_x = u - 0.7;
    let spot_y = v - 0.4;
    let spot_dist = (spot_x * spot_x + spot_y * spot_y).sqrt();
    let red_spot = Vector3::new(1.0, 0.3, 0.0);
    let result = mix_color(with_storm, red_spot, smoothstep(0.15, 0.05, spot_dist) * 0.8);
    
    result
}

/// MOON SHADER - Gray/Rocky surface (for Earth's Moon, etc)
fn moon_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    let pos = vertex.transformed_position;
    let len = (pos.x * pos.x + pos.y * pos.y + pos.z * pos.z).sqrt();
    if len < 0.001 {
        return Vector3::new(0.0, 0.0, 0.0);
    }
    
    let norm = Vector3::new(pos.x / len, pos.y / len, pos.z / len);
    let u = (norm.x.atan2(norm.z) / std::f32::consts::PI + 1.0) * 0.5;
    let v = (norm.y).asin() / std::f32::consts::PI + 0.5;
    
    let uv = Vector2::new(u, v);
    
    // Layer 1: Base gray rocky surface
    let base = Vector3::new(0.5, 0.5, 0.52);
    
    // Layer 2: Cratered surface
    let craters = fbm(uv * 8.0, 4);
    let crater_shadows = (craters - 0.3) * 2.0;
    let surface = mix_color(base, Vector3::new(0.3, 0.3, 0.32), crater_shadows.clamp(0.0, 1.0) * 0.7);
    
    // Layer 3: Bright highlights on peaks
    let peaks = fbm(uv * 15.0 + time * 0.1, 2);
    let highlights = mix_color(surface, Vector3::new(0.8, 0.8, 0.82), (peaks - 0.5) * 0.4);
    
    // Layer 4: Subtle color variations
    let variations = fbm(uv * 3.0, 2);
    let result = mix_color(highlights, Vector3::new(0.6, 0.6, 0.58), variations * 0.2);
    
    result
}

/// RING SHADER - Saturn-like rings with bands
fn ring_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
    let u = vertex.tex_coords.x;
    let v = vertex.tex_coords.y;
    
    // Distance from center (0 = inner, 1 = outer)
    let r = (u - 0.5) * (u - 0.5) + (v - 0.5) * (v - 0.5);
    let r = r.sqrt() * 2.0;
    
    // Layer 1: Base ring color (pale gold)
    let base = Vector3::new(0.9, 0.85, 0.6);
    
    // Layer 2: Ring bands (alternating darker and lighter bands)
    let bands = ((r * 30.0).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_color = Vector3::new(0.7, 0.6, 0.3);
    let with_bands = mix_color(base, band_color, bands * 0.5);
    
    // Layer 3: Particle shadows
    let particles = fbm(Vector2::new(u * 10.0, r * 20.0 + time * 0.5), 3);
    let shadow = mix_color(with_bands, Vector3::new(0.5, 0.4, 0.1), particles * 0.4);
    
    // Layer 4: Edge darker (depth effect)
    let edge_darkness = smoothstep(0.0, 0.2, r) * smoothstep(1.5, 1.0, r);
    let result = mix_color(shadow, Vector3::new(0.2, 0.15, 0.05), (1.0 - edge_darkness) * 0.6);
    
    result
}

/// Get the appropriate shader color based on planet type
pub fn get_planet_color(fragment: &Fragment, vertex: &Vertex, time: f32, planet_type: u32) -> Vector3 {
    match planet_type {
        0 => sun_shader(fragment, vertex, time),
        1 => earth_shader(fragment, vertex, time),
        2 => gas_giant_shader(fragment, vertex, time),
        3 => moon_shader(fragment, vertex, time),    // Moon shader
        4 => ring_shader(fragment, vertex, time),    // Ring shader
        _ => Vector3::new(1.0, 1.0, 1.0), // Default white
    }
}