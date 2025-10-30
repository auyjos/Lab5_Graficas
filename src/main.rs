// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;
mod rings;
mod moons;
mod texture;

use crate::matrix::new_matrix4;
use crate::shaders::get_planet_color;
use crate::texture::Texture;
use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::vertex_shader;
use obj::Obj;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

pub struct Uniforms {
    pub model_matrix: Matrix,
    pub time: f32,
    pub planet_type: u32,  // 0: Sun, 1: Earth-like, 2: Gas Giant
}

struct CelestialBody {
    #[allow(dead_code)]
    name: String,
    planet_type: u32,
    scale: f32,
    orbit_radius: f32,
    orbit_speed: f32,
    rotation_speed: f32,
    model_path: String, // Path to OBJ file
}

fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    // Rotation around the X-axis
    let rotation_matrix_x = new_matrix4(
        1.0, 0.0,    0.0,    0.0,
        0.0, cos_x,  -sin_x, 0.0,
        0.0, sin_x,  cos_x,  0.0,
        0.0, 0.0,    0.0,    1.0
    );

    // Rotation around the Y-axis
    let rotation_matrix_y = new_matrix4(
        cos_y,  0.0, sin_y, 0.0,
        0.0,    1.0, 0.0,   0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0,    0.0, 0.0,   1.0
    );

    // Rotation around the Z-axis
    let rotation_matrix_z = new_matrix4(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z,  0.0, 0.0,
        0.0,   0.0,    1.0, 0.0,
        0.0,   0.0,    0.0, 1.0
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    // Scaling matrix
    let scale_matrix = new_matrix4(
        scale, 0.0,   0.0,   0.0,
        0.0,   scale, 0.0,   0.0,
        0.0,   0.0,   scale, 0.0,
        0.0,   0.0,   0.0,   1.0
    );

    // Translation matrix
    let translation_matrix = new_matrix4(
        1.0, 0.0, 0.0, translation.x,
        0.0, 1.0, 0.0, translation.y,
        0.0, 0.0, 1.0, translation.z,
        0.0, 0.0, 0.0, 1.0
    );

    scale_matrix * rotation_matrix * translation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        // Create a temporary vertex at the fragment position for shader evaluation
        let temp_vertex = Vertex {
            position: Vector3::new(fragment.position.x, fragment.position.y, 0.0),
            normal: Vector3::new(0.0, 1.0, 0.0),
            tex_coords: Vector2::zero(),
            color: fragment.color, // Use material color from the vertex
            transformed_position: Vector3::new(fragment.position.x, fragment.position.y, fragment.depth),
            transformed_normal: Vector3::new(0.0, 1.0, 0.0),
        };
        
        // Apply shader to get color based on planet type
        let color = get_planet_color(&fragment, &temp_vertex, uniforms.time, uniforms.planet_type);
        
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            color
        );
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Rust Graphics - Renderer Example")
        .log_level(TraceLogLevel::LOG_WARNING) // Suppress INFO messages
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.01, 0.01, 0.05)); // Deep space black with slight blue tint

    // Initialize the texture inside the framebuffer
    framebuffer.init_texture(&mut window, &thread);

    // Animation parameters
    let mut time = 0.0f32;
    let mut auto_rotate = true;
    let mut auto_orbit = true;
    
    // Camera/viewport control
    let mut camera_offset = Vector3::new(0.0, 0.0, 0.0);
    let mut camera_zoom = 0.6f32;  // Start more zoomed out to see all planets
    let mut system_rotation = Vector3::new(0.0, 0.0, 0.0);

    // Load all unique models into a cache (with textures)
    let mut model_cache: std::collections::HashMap<String, (Vec<Vertex>, Option<Texture>)> = std::collections::HashMap::new();
    
    // Pre-load unique models
    let unique_models = vec![
        "assets/models/13913_Sun_v2_l3.obj",
        "assets/models/13902_Earth_v1_l3.obj",
        "assets/models/13905_Jupiter_V1_l3.obj",
        "assets/models/13907_Uranus_v2_l3.obj",
        "assets/models/10464_Asteroid_v1_Iterations-2.obj",
    ];
    
    for model_path in unique_models {
        match Obj::load(model_path) {
            Ok(obj) => {
                let vertex_array = obj.get_vertex_array();
                let texture = obj.get_texture().clone();
                model_cache.insert(model_path.to_string(), (vertex_array, texture));
                println!("✓ Loaded model: {}", model_path);
            }
            Err(e) => {
                eprintln!("✗ Failed to load {}: {:?}", model_path, e);
            }
        }
    }

    // Define celestial bodies with their respective OBJ models
    let bodies = vec![
        CelestialBody {
            name: "Sol".to_string(),
            planet_type: 0,
            scale: 50.0,  // Slightly larger sun
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            rotation_speed: 0.02,
            model_path: "assets/models/13913_Sun_v2_l3.obj".to_string(),
        },
        CelestialBody {
            name: "Tierra".to_string(),
            planet_type: 1,
            scale: 20.0,  // Smaller to emphasize distance
            orbit_radius: 180.0,  // Much farther from sun
            orbit_speed: 0.15,
            rotation_speed: 0.03,
            model_path: "assets/models/13902_Earth_v1_l3.obj".to_string(),
        },
        CelestialBody {
            name: "Gigante Gaseoso".to_string(),
            planet_type: 2,
            scale: 28.0,  // Smaller but still large
            orbit_radius: 280.0,  // Very far orbit
            orbit_speed: 0.08,
            rotation_speed: 0.02,
            model_path: "assets/models/13905_Jupiter_V1_l3.obj".to_string(),
        },
        CelestialBody {
            name: "Venus".to_string(),
            planet_type: 7,
            scale: 18.0,  // Smaller, closer to sun
            orbit_radius: 120.0,  // Between sun and Earth
            orbit_speed: 0.20,
            rotation_speed: 0.008,
            model_path: "assets/models/13902_Earth_v1_l3.obj".to_string(), // Reuse Earth model
        },
        CelestialBody {
            name: "Neptuno".to_string(),
            planet_type: 5,
            scale: 25.0,  // Medium size
            orbit_radius: 380.0,  // Very far orbit
            orbit_speed: 0.05,
            rotation_speed: 0.025,
            model_path: "assets/models/13907_Uranus_v2_l3.obj".to_string(), // Reuse Uranus model
        },
        CelestialBody {
            name: "Urano".to_string(),
            planet_type: 6,
            scale: 24.0,  // Similar to Neptune
            orbit_radius: 330.0,  // Between Jupiter and Neptune
            orbit_speed: 0.07,
            rotation_speed: 0.035,
            model_path: "assets/models/13907_Uranus_v2_l3.obj".to_string(),
        },
    ];

    while !window.window_should_close() {
        handle_input(&mut window, &mut camera_offset, &mut camera_zoom, &mut system_rotation, &mut auto_rotate, &mut auto_orbit);

        // Update time
        time += 0.016; // Approximately 60 FPS

        framebuffer.clear();

        // Center point for the solar system (affected by camera offset)
        let center = Vector3::new(400.0 + camera_offset.x, 300.0 + camera_offset.y, 0.0 + camera_offset.z);

        // Render all celestial bodies
        for body in bodies.iter() {
            // Calculate position
            let body_rotation = if auto_rotate {
                Vector3::new(0.0, time * body.rotation_speed, 0.0)
            } else {
                Vector3::new(0.0, 0.0, 0.0)
            };

            let body_translation = if auto_orbit {
                let orbit_angle = time * body.orbit_speed;
                // Create a proper 3D elliptical orbit with inclination
                // Each planet has different orbital characteristics
                let inclination = body.planet_type as f32 * 0.4; // Stronger inclination per planet
                
                // Primary orbit in X-Y plane
                let orbit_x = orbit_angle.cos() * body.orbit_radius;
                let orbit_y = orbit_angle.sin() * body.orbit_radius;
                
                // Z component (vertical oscillation due to orbit inclination)
                // The Z position changes as the planet orbits
                let orbit_z = (orbit_angle * inclination).sin() * body.orbit_radius * 0.5;
                
                Vector3::new(
                    center.x + orbit_x,
                    center.y + orbit_y,
                    center.z + orbit_z,
                )
            } else {
                center
            };

            // Apply system-wide rotation around center
            let rotated_translation = rotate_point_around_center(body_translation, center, system_rotation);

            let model_matrix = create_model_matrix(rotated_translation, body.scale * camera_zoom, body_rotation);
            let uniforms = Uniforms {
                model_matrix,
                time,
                planet_type: body.planet_type,
            };

            // Get the vertex array for this body's model
            if let Some((vertex_array, _texture)) = model_cache.get(&body.model_path) {
                render(&mut framebuffer, &uniforms, vertex_array);
            }

            // Renderizar lunas y anillos específicos para algunos planetas
            match body.planet_type {
                1 => {
                    // Tierra - Renderizar luna (la Luna)
                    let moon_orbit_angle = time * 0.08; // Faster moon orbit
                    let moon_distance = 50.0;
                    let moon_pos = Vector3::new(
                        rotated_translation.x + moon_orbit_angle.cos() * moon_distance,
                        rotated_translation.y + moon_orbit_angle.sin() * moon_distance,
                        rotated_translation.z,
                    );
                    
                    let moon_rotation = if auto_rotate {
                        Vector3::new(0.0, time * 0.05, 0.0)
                    } else {
                        Vector3::new(0.0, 0.0, 0.0)
                    };
                    
                    let moon_matrix = create_model_matrix(moon_pos, 8.0 * camera_zoom, moon_rotation);
                    let moon_uniforms = Uniforms {
                        model_matrix: moon_matrix,
                        time,
                        planet_type: 3, // Moon shader
                    };
                    
                    // Use asteroid model for moon
                    if let Some((moon_vertex_array, _moon_texture)) = model_cache.get("assets/models/10464_Asteroid_v1_Iterations-2.obj") {
                        render(&mut framebuffer, &moon_uniforms, moon_vertex_array);
                    }
                },
                2 => {
                    // Gigante Gaseoso - Renderizar anillos
                    let ring_scale = 1.8;
                    let ring_matrix = create_model_matrix(rotated_translation, body.scale * ring_scale * camera_zoom, Vector3::new(0.2, 0.0, 0.0));
                    let ring_uniforms = Uniforms {
                        model_matrix: ring_matrix,
                        time,
                        planet_type: 4, // Ring shader
                    };
                    
                    // Generate and render ring geometry
                    let ring_vertices = rings::generate_flat_ring(1.0, 1.5, 128);
                    render(&mut framebuffer, &ring_uniforms, &ring_vertices);
                },
                _ => {}
            }
        }

        // Display framebuffer and text overlay
        framebuffer.update_texture();
        
        let mut draw_handle = window.begin_drawing(&thread);
        draw_handle.clear_background(Color::BLACK);
        framebuffer.draw(&mut draw_handle);
        
        // Draw HUD - Top info
        draw_handle.draw_text(&format!("FPS: {}", draw_handle.get_fps()), 10, 10, 20, Color::GREEN);
        draw_handle.draw_text("Sistema Solar - 6 Cuerpos Celestes", 10, 40, 20, Color::WHITE);
        draw_handle.draw_text(&format!("Time: {:.1}s", time), 10, 70, 15, Color::GRAY);
        
        // Show status with clear indicators
        let status_rotate = if auto_rotate { "▶ ACTIVA" } else { "⏸ PAUSADA" };
        let status_orbit = if auto_orbit { "▶ ACTIVA" } else { "⏸ PAUSADA" };
        draw_handle.draw_text(&format!("Rotación: {} | Órbita: {}", status_rotate, status_orbit), 10, 100, 14, Color::YELLOW);
        
        // Draw HUD - Bottom controls
        let y_offset = window_height as i32 - 150;
        draw_handle.draw_text("CONTROLES:", 10, y_offset, 18, Color::YELLOW);
        draw_handle.draw_text("SPACE: Pausar/Reanudar rotacion", 10, y_offset + 25, 14, Color::LIGHTGRAY);
        draw_handle.draw_text("O: Pausar/Reanudar orbita", 10, y_offset + 45, 14, Color::LIGHTGRAY);
        draw_handle.draw_text("Flechas: Mover camara | S/A: Zoom", 10, y_offset + 65, 14, Color::LIGHTGRAY);
        draw_handle.draw_text("Q/W: Rot X | E/R: Rot Y | T/Y: Rot Z", 10, y_offset + 85, 14, Color::LIGHTGRAY);
        draw_handle.draw_text(&format!("Zoom: {:.2}x", camera_zoom), 10, y_offset + 110, 14, Color::LIGHTGRAY);

        thread::sleep(Duration::from_millis(16));
    }
}

// Helper function to rotate a point around a center point
fn rotate_point_around_center(point: Vector3, center: Vector3, rotation: Vector3) -> Vector3 {
    // Translate to origin
    let p = Vector3::new(point.x - center.x, point.y - center.y, point.z - center.z);
    
    // Apply rotation matrices
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();
    
    // Rotate around X
    let p = Vector3::new(p.x, p.y * cos_x - p.z * sin_x, p.y * sin_x + p.z * cos_x);
    
    // Rotate around Y
    let p = Vector3::new(p.x * cos_y + p.z * sin_y, p.y, -p.x * sin_y + p.z * cos_y);
    
    // Rotate around Z
    let p = Vector3::new(p.x * cos_z - p.y * sin_z, p.x * sin_z + p.y * cos_z, p.z);
    
    // Translate back
    Vector3::new(p.x + center.x, p.y + center.y, p.z + center.z)
}

fn handle_input(
    window: &mut RaylibHandle,
    camera_offset: &mut Vector3,
    camera_zoom: &mut f32,
    system_rotation: &mut Vector3,
    auto_rotate: &mut bool,
    auto_orbit: &mut bool,
) {
    // Camera movement (arrow keys)
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        camera_offset.x += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        camera_offset.x -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        camera_offset.y -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        camera_offset.y += 10.0;
    }
    
    // Zoom (S/A keys)
    if window.is_key_down(KeyboardKey::KEY_S) {
        *camera_zoom += 0.05;
        if *camera_zoom > 3.0 { *camera_zoom = 3.0; }
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        *camera_zoom -= 0.05;
        if *camera_zoom < 0.3 { *camera_zoom = 0.3; }
    }
    
    // System rotation (Q/W/E/R/T/Y keys)
    if window.is_key_down(KeyboardKey::KEY_Q) {
        system_rotation.x -= PI / 30.0;
    }
    if window.is_key_down(KeyboardKey::KEY_W) {
        system_rotation.x += PI / 30.0;
    }
    if window.is_key_down(KeyboardKey::KEY_E) {
        system_rotation.y -= PI / 30.0;
    }
    if window.is_key_down(KeyboardKey::KEY_R) {
        system_rotation.y += PI / 30.0;
    }
    if window.is_key_down(KeyboardKey::KEY_T) {
        system_rotation.z -= PI / 30.0;
    }
    if window.is_key_down(KeyboardKey::KEY_Y) {
        system_rotation.z += PI / 30.0;
    }
    
    // Toggle auto-rotation with SPACE
    if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
        *auto_rotate = !*auto_rotate;
    }
    
    // Toggle auto-orbit with O
    if window.is_key_pressed(KeyboardKey::KEY_O) {
        *auto_orbit = !*auto_orbit;
    }
}
