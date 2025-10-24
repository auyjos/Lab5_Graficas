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

/// SUN SHADER - Dynamic solar surface with 5 layers
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
    
    // Layer 1: Core temperature gradient (white-yellow-orange)
    let core_gradient = Vector3::new(1.0, 1.0, 0.2) * (0.8 + fbm(uv * 2.0, 2) * 0.2);
    
    // Layer 2: Photosphere turbulence (thick noise patterns)
    let photosphere = fbm(uv * 6.0 + time * 0.25, 4);
    let photosphere_color = Vector3::new(1.0, 0.7, 0.0);
    let with_photosphere = mix_color(core_gradient, photosphere_color, photosphere * 0.6);
    
    // Layer 3: Solar prominences (bright streaks)
    let prominences = fbm(uv * 8.0 - time * 0.15, 3);
    let prominence_height = (uv.y - 0.5).abs() * 2.0;
    let prominence_effect = (1.0 - prominence_height) * prominences;
    let prominence_color = Vector3::new(1.0, 0.9, 0.3);
    let with_prominences = mix_color(with_photosphere, prominence_color, prominence_effect * 0.4);
    
    // Layer 4: Corona glow (outer atmosphere)
    let corona_pattern = fbm(uv * 12.0 + time * 0.3, 2);
    let corona_radius = ((uv.x - 0.5) * (uv.x - 0.5) + (uv.y - 0.5) * (uv.y - 0.5)).sqrt();
    let corona_glow = (0.5 - corona_radius).clamp(0.0, 0.3) * corona_pattern;
    let corona_color = Vector3::new(1.0, 0.95, 0.7);
    let with_corona = mix_color(with_prominences, corona_color, corona_glow * 0.5);
    
    // Layer 5: Magnetic field lines (flowing patterns)
    let magnetic_fields = (time.sin() * (uv.x * 25.0 + time * 0.1).sin() * (uv.y * 15.0).cos()).abs();
    let magnetic_color = Vector3::new(1.0, 1.0, 0.4);
    let result = mix_color(with_corona, magnetic_color, magnetic_fields * 0.15);
    
    result
}

/// EARTH-LIKE PLANET - Realistic with 5 layers (oceans, continents, clouds, ice, atmosphere)
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
    
    // Layer 1: Ocean base (deep blue)
    let ocean_base = Vector3::new(0.0, 0.3, 0.7);
    
    // Layer 2: Landmasses (continents using complex noise)
    let land_noise = fbm(uv * 5.0 - time * 0.02, 5);
    let land_mask = smoothstep(0.4, 0.6, land_noise);
    let land_color = mix_color(
        Vector3::new(0.0, 0.5, 0.0),  // Forest green
        Vector3::new(0.8, 0.7, 0.3),  // Desert sand
        fbm(uv * 15.0, 2)
    );
    let with_land = mix_color(ocean_base, land_color, land_mask);
    
    // Layer 3: Mountain ranges (bright peaks on continents)
    let mountain_detail = fbm(uv * 25.0, 3);
    let mountain_mask = land_mask * (mountain_detail - 0.3).clamp(0.0, 1.0);
    let mountain_color = Vector3::new(0.6, 0.5, 0.4);
    let with_mountains = mix_color(with_land, mountain_color, mountain_mask * 0.7);
    
    // Layer 4: Clouds (animated swirling patterns)
    let cloud_noise1 = fbm(uv * 4.0 + time * 0.05, 3);
    let cloud_noise2 = fbm(uv * 6.0 - time * 0.03, 3);
    let clouds = smoothstep(0.3, 0.8, (cloud_noise1 + cloud_noise2) * 0.5);
    let cloud_color = Vector3::new(1.0, 1.0, 1.0);
    let with_clouds = mix_color(with_mountains, cloud_color, clouds * 0.6);
    
    // Layer 5: Polar ice caps and atmospheric glow
    let ice_factor = (1.0 - (v - 0.5).abs() * 3.0).clamp(0.0, 1.0);
    let ice_color = Vector3::new(0.9, 0.95, 1.0);
    let with_ice = mix_color(with_clouds, ice_color, ice_factor * 0.4);
    
    // Atmospheric rim (subtle blue glow at edges)
    let rim_factor = smoothstep(0.8, 1.0, ((uv.x - 0.5) * (uv.x - 0.5) + (uv.y - 0.5) * (uv.y - 0.5)).sqrt() * 1.5);
    let atmosphere_color = Vector3::new(0.3, 0.6, 1.0);
    let result = mix_color(with_ice, atmosphere_color, rim_factor * 0.3);
    
    result
}

/// GAS GIANT - Complex with 5 layers (bands, storms, great red spot, lightning, atmospheric depth)
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
    
    // Layer 1: Base atmosphere color (orange/brown gradient)
    let base_color = mix_color(
        Vector3::new(1.0, 0.6, 0.2),  // Bright orange
        Vector3::new(0.8, 0.4, 0.1),  // Dark brown
        v  // Gradient from bottom to top
    );
    
    // Layer 2: Atmospheric bands (horizontal stripes)
    let bands = ((v * 20.0 + time * 0.1).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_darkness = smoothstep(0.3, 0.6, bands);
    let band_color = Vector3::new(0.6, 0.3, 0.1);
    let with_bands = mix_color(base_color, band_color, band_darkness * 0.5);
    
    // Layer 3: Turbulent storms and wind patterns
    let storm_noise1 = fbm(uv * 8.0 + time * 0.08, 4);
    let storm_noise2 = fbm(uv * 5.0 - time * 0.12, 3);
    let storms = (storm_noise1 + storm_noise2) * 0.5;
    let storm_mask = smoothstep(0.2, 0.8, storms);
    let storm_color = mix_color(
        Vector3::new(0.7, 0.4, 0.1),
        Vector3::new(0.4, 0.2, 0.0),
        fbm(uv * 15.0, 2)
    );
    let with_storms = mix_color(with_bands, storm_color, storm_mask * 0.6);
    
    // Layer 4: Great Red Spot (massive storm system)
    let spot_center_x = 0.6;
    let spot_center_y = 0.35;
    let spot_x = u - spot_center_x;
    let spot_y = v - spot_center_y;
    let spot_dist = (spot_x * spot_x + spot_y * spot_y).sqrt();
    
    let spot_swirl = fbm(Vector2::new(u * 10.0 + spot_dist * 20.0 - time * 0.1, v * 5.0), 3);
    let red_spot_color = mix_color(
        Vector3::new(1.0, 0.3, 0.0),   // Bright red
        Vector3::new(0.8, 0.1, 0.0),   // Deep red
        spot_swirl
    );
    let spot_effect = smoothstep(0.2, 0.05, spot_dist);
    let with_spot = mix_color(with_storms, red_spot_color, spot_effect * 0.9);
    
    // Layer 5: Lightning and atmospheric disturbances
    let lightning_x = (u * 50.0 + time * 0.3).sin() * 0.1;
    let lightning_y = (v * 40.0 - time * 0.25).sin() * 0.1;
    let lightning_intensity = ((lightning_x + lightning_y).abs() - 0.1).clamp(0.0, 0.2);
    let lightning_color = Vector3::new(1.0, 1.0, 0.3);
    let result = mix_color(with_spot, lightning_color, lightning_intensity * 0.3);
    
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

/// NEPTUNE - Deep blue with dynamic storms and white clouds
fn neptune_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
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
    
    // Layer 1: Base deep ocean blue
    let base_color = mix_color(
        Vector3::new(0.0, 0.2, 0.6),  // Deep navy
        Vector3::new(0.1, 0.3, 0.9),  // Vivid blue
        v * 0.7
    );
    
    // Layer 2: Methane cloud bands
    let cloud_bands = ((v * 15.0 - time * 0.08).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_noise = fbm(uv * 4.0, 2);
    let cloud_mask = smoothstep(0.3, 0.7, cloud_bands + band_noise * 0.3);
    let with_clouds = mix_color(base_color, Vector3::new(0.9, 0.95, 1.0), cloud_mask * 0.4);
    
    // Layer 3: Great Dark Spot (storm system similar to Jupiter)
    let spot_center_x = 0.5;
    let spot_center_y = 0.5;
    let spot_x = (u - spot_center_x) * (u - spot_center_x);
    let spot_y = (v - spot_center_y - 0.1) * (v - spot_center_y - 0.1);
    let spot_dist = (spot_x + spot_y).sqrt();
    
    let spot_interior = fbm(uv * 12.0 + time * 0.15, 3);
    let dark_spot = mix_color(
        Vector3::new(0.0, 0.1, 0.3),  // Dark blue center
        Vector3::new(0.1, 0.2, 0.5),  // Lighter blue edges
        spot_interior
    );
    let spot_effect = smoothstep(0.25, 0.05, spot_dist);
    let with_spot = mix_color(with_clouds, dark_spot, spot_effect * 0.8);
    
    // Layer 4: High-altitude white streaks (fast winds)
    let wind_streak = (u * 30.0 - time * 0.4).sin() * 0.5 + 0.5;
    let streak_mask = smoothstep(0.3, 0.55, v) * smoothstep(0.65, 0.55, v);
    let white_streaks = Vector3::new(1.0, 1.0, 1.0);
    let with_streaks = mix_color(with_spot, white_streaks, (wind_streak.abs() - 0.3) * streak_mask * 0.3);
    
    // Layer 5: Atmospheric turbulence and depth
    let turbulence = fbm(uv * 7.0 - time * 0.12, 4);
    let depth_color = Vector3::new(0.0, 0.1, 0.4);
    let result = mix_color(with_streaks, depth_color, turbulence * 0.15);
    
    result
}

/// URANUS - Cyan ice giant with tilted appearance and icy rings
fn uranus_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
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
    
    // Layer 1: Base icy cyan color
    let base_color = mix_color(
        Vector3::new(0.3, 0.8, 0.9),  // Bright cyan
        Vector3::new(0.2, 0.6, 0.8),  // Darker cyan
        fbm(uv * 2.0, 2)
    );
    
    // Layer 2: Methane frost patterns
    let frost = fbm(uv * 6.0 + time * 0.05, 3);
    let frost_color = Vector3::new(0.6, 0.95, 1.0);
    let with_frost = mix_color(base_color, frost_color, frost * 0.6);
    
    // Layer 3: Subtle polar bands (unlike other planets, Uranus has faint bands)
    let polar_bands = ((v * 8.0).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_color = mix_color(
        Vector3::new(0.2, 0.5, 0.7),
        Vector3::new(0.4, 0.9, 1.0),
        fbm(uv * 10.0, 2)
    );
    let with_bands = mix_color(with_frost, band_color, polar_bands * 0.3);
    
    // Layer 4: Tilted storm spot (Uranus rotates on its side)
    let tilted_u = u - 0.3 * (time * 0.1).sin();
    let tilted_v = v - 0.5 + 0.2 * (time * 0.08).cos();
    let storm_x = (tilted_u - 0.5) * (tilted_u - 0.5);
    let storm_y = (tilted_v - 0.3) * (tilted_v - 0.3);
    let storm_dist = (storm_x + storm_y).sqrt();
    
    let storm_interior = fbm(uv * 14.0 + time * 0.2, 3);
    let storm_color = mix_color(
        Vector3::new(0.1, 0.4, 0.6),
        Vector3::new(0.5, 0.9, 1.0),
        storm_interior
    );
    let storm_effect = smoothstep(0.2, 0.04, storm_dist);
    let with_storm = mix_color(with_bands, storm_color, storm_effect * 0.9);
    
    // Layer 5: Icy gloss and atmospheric shimmer
    let gloss = fbm(uv * 20.0 - time * 0.3, 2);
    let shimmer = smoothstep(0.4, 0.6, gloss);
    let shine_color = Vector3::new(1.0, 1.0, 1.0);
    let result = mix_color(with_storm, shine_color, shimmer * 0.2);
    
    result
}

/// VENUS - Hellish planet with thick yellow atmosphere and volcanic patterns
fn venus_shader(_fragment: &Fragment, vertex: &Vertex, time: f32) -> Vector3 {
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
    
    // Layer 1: Base hellish yellow atmosphere
    let base_color = mix_color(
        Vector3::new(1.0, 0.9, 0.4),  // Bright yellow
        Vector3::new(1.0, 0.8, 0.3),  // Orange-yellow
        fbm(uv * 3.0, 2) * 0.5
    );
    
    // Layer 2: Thick toxic cloud swirls
    let cloud_swirl1 = fbm(uv * 4.0 + time * 0.15, 3);
    let cloud_swirl2 = fbm(uv * 5.0 - time * 0.1, 3);
    let clouds = mix_color(
        Vector3::new(1.0, 0.85, 0.2),
        Vector3::new(0.8, 0.6, 0.0),
        (cloud_swirl1 + cloud_swirl2) * 0.5
    );
    let with_clouds = mix_color(base_color, clouds, 0.7);
    
    // Layer 3: Volcanic hot spots
    let volcano1 = fbm(uv * 8.0 + time * 0.05, 2);
    let volcano2 = fbm((uv + Vector2::new(0.3, 0.4)) * 6.0 - time * 0.08, 2);
    let hot_spot_color = mix_color(
        Vector3::new(1.0, 0.3, 0.0),  // Red hot
        Vector3::new(1.0, 0.5, 0.0),  // Orange hot
        (volcano1 + volcano2) * 0.5
    );
    let hot_mask = ((volcano1 - 0.3) * 3.0).clamp(0.0, 1.0);
    let with_volcanoes = mix_color(with_clouds, hot_spot_color, hot_mask * 0.5);
    
    // Layer 4: Atmospheric banding (super-rotation patterns)
    let super_rotate = ((v * 20.0 + u * 3.0 - time * 0.2).sin() * 0.5 + 0.5).max(0.0).min(1.0);
    let band_noise = fbm(uv * 12.0, 2);
    let band_color = Vector3::new(1.0, 0.7, 0.1);
    let with_bands = mix_color(with_volcanoes, band_color, super_rotate * band_noise * 0.3);
    
    // Layer 5: Atmospheric glow at edges (greenhouse effect)
    let rim = (1.0 - v.abs() * 2.0 - 0.3).clamp(0.0, 1.0);
    let glow_color = Vector3::new(1.0, 0.4, 0.0);
    let result = mix_color(with_bands, glow_color, rim * rim * 0.3);
    
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
        5 => neptune_shader(fragment, vertex, time), // Neptune shader
        6 => uranus_shader(fragment, vertex, time),  // Uranus shader
        7 => venus_shader(fragment, vertex, time),   // Venus shader
        _ => Vector3::new(1.0, 1.0, 1.0), // Default white
    }
}