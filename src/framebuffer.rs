use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    image: Image,
    background_color: Vector3,
    texture: Option<Texture2D>,
    star_field: Vec<(i32, i32, f32)>, // (x, y, brightness)
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let image = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        
        // Generate star field
        let star_field = Self::generate_stars(width, height);
        
        Framebuffer {
            width,
            height,
            image,
            background_color: Vector3::zero(),
            texture: None,
            star_field,
        }
    }
    
    fn generate_stars(width: u32, height: u32) -> Vec<(i32, i32, f32)> {
        let mut stars = Vec::new();
        let star_count = 800; // Number of stars
        
        // Simple pseudo-random number generator using LCG
        let mut seed = 12345u64;
        let a = 1103515245u64;
        let c = 12345u64;
        let m = 2u64.pow(31);
        
        for _ in 0..star_count {
            // Generate pseudo-random x
            seed = (a.wrapping_mul(seed).wrapping_add(c)) % m;
            let x = (seed % width as u64) as i32;
            
            // Generate pseudo-random y
            seed = (a.wrapping_mul(seed).wrapping_add(c)) % m;
            let y = (seed % height as u64) as i32;
            
            // Generate pseudo-random brightness (0.3 to 1.0)
            seed = (a.wrapping_mul(seed).wrapping_add(c)) % m;
            let brightness = 0.3 + (seed % 70) as f32 / 100.0;
            
            stars.push((x, y, brightness));
        }
        
        stars
    }

    pub fn init_texture(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        self.texture = Some(rl.load_texture_from_image(thread, &self.image).unwrap());
    }

    pub fn clear(&mut self) {
        // Draw deep space background
        let bg_color = Color::new(
            (self.background_color.x * 255.0) as u8,
            (self.background_color.y * 255.0) as u8,
            (self.background_color.z * 255.0) as u8,
            255,
        );
        self.image.clear_background(bg_color);
        
        // Draw stars
        for &(x, y, brightness) in &self.star_field {
            let star_color = Color::new(
                (255.0 * brightness) as u8,
                (255.0 * brightness) as u8,
                (255.0 * brightness * 0.9) as u8, // Slight blue tint
                255,
            );
            self.image.draw_pixel(x, y, star_color);
            
            // Draw some larger stars (about 10% of them)
            if brightness > 0.8 {
                // Draw a small cross pattern for brighter stars
                if x > 0 {
                    self.image.draw_pixel(x - 1, y, Color::new(
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.45) as u8,
                        255,
                    ));
                }
                if x < self.width as i32 - 1 {
                    self.image.draw_pixel(x + 1, y, Color::new(
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.45) as u8,
                        255,
                    ));
                }
                if y > 0 {
                    self.image.draw_pixel(x, y - 1, Color::new(
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.45) as u8,
                        255,
                    ));
                }
                if y < self.height as i32 - 1 {
                    self.image.draw_pixel(x, y + 1, Color::new(
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.5) as u8,
                        (255.0 * brightness * 0.45) as u8,
                        255,
                    ));
                }
            }
        }
    }

    pub fn point(&mut self, x: i32, y: i32, color: Vector3) {
        if x >= 0 && y >= 0 && x < self.width as i32 && y < self.height as i32 {
            let pixel_color = Color::new(
                (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                255,
            );
            self.image.draw_pixel(x, y, pixel_color);
        }
    }

    pub fn set_background_color(&mut self, color: Vector3) {
        self.background_color = color;
    }

    pub fn update_texture(&mut self) {
        if let Some(texture) = &mut self.texture {
            let colors = self.image.get_image_data();
            // Safely cast the &[Color] slice to a &[u8] slice for the update function
            let data: &[u8] = unsafe {
                std::slice::from_raw_parts(
                    colors.as_ptr() as *const u8,
                    colors.len() * 4, // Each Color is 4 bytes (r,g,b,a)
                )
            };
            texture.update_texture(data).unwrap();
        } else {
            panic!("Framebuffer texture has not been initialized. Call init_texture after creating the RaylibHandle.");
        }
    }

    pub fn draw(&self, draw_handle: &mut RaylibDrawHandle) {
        if let Some(texture) = &self.texture {
            draw_handle.draw_texture(texture, 0, 0, Color::WHITE);
        }
    }
}