use raylib::math::Vector3;
use image::{DynamicImage, GenericImageView};
use std::path::Path;

#[derive(Clone)]
pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA data
}

impl Texture {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let img = image::open(path)?;
        let rgba = img.to_rgba8();
        let (width, height) = rgba.dimensions();
        
        Ok(Texture {
            width,
            height,
            data: rgba.into_raw(),
        })
    }

    /// Sample texture at UV coordinates (0.0 - 1.0 range)
    pub fn sample(&self, u: f32, v: f32) -> Vector3 {
        // Wrap UV coordinates
        let u = u.fract();
        let v = v.fract();
        
        // Convert to pixel coordinates
        let x = ((u * self.width as f32) as u32).min(self.width - 1);
        let y = ((v * self.height as f32) as u32).min(self.height - 1);
        
        // Get pixel index (RGBA format = 4 bytes per pixel)
        let idx = ((y * self.width + x) * 4) as usize;
        
        if idx + 2 < self.data.len() {
            Vector3::new(
                self.data[idx] as f32 / 255.0,
                self.data[idx + 1] as f32 / 255.0,
                self.data[idx + 2] as f32 / 255.0,
            )
        } else {
            Vector3::new(1.0, 1.0, 1.0)
        }
    }

    /// Sample with bilinear filtering for smoother results
    pub fn sample_bilinear(&self, u: f32, v: f32) -> Vector3 {
        let u = u.fract();
        let v = v.fract();
        
        let x = u * self.width as f32 - 0.5;
        let y = v * self.height as f32 - 0.5;
        
        let x0 = x.floor().max(0.0) as u32;
        let y0 = y.floor().max(0.0) as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        
        let fx = x - x.floor();
        let fy = y - y.floor();
        
        let c00 = self.get_pixel(x0, y0);
        let c10 = self.get_pixel(x1, y0);
        let c01 = self.get_pixel(x0, y1);
        let c11 = self.get_pixel(x1, y1);
        
        // Bilinear interpolation
        let c0 = mix_vec3(c00, c10, fx);
        let c1 = mix_vec3(c01, c11, fx);
        mix_vec3(c0, c1, fy)
    }
    
    fn get_pixel(&self, x: u32, y: u32) -> Vector3 {
        let idx = ((y * self.width + x) * 4) as usize;
        if idx + 2 < self.data.len() {
            Vector3::new(
                self.data[idx] as f32 / 255.0,
                self.data[idx + 1] as f32 / 255.0,
                self.data[idx + 2] as f32 / 255.0,
            )
        } else {
            Vector3::new(1.0, 1.0, 1.0)
        }
    }
}

fn mix_vec3(a: Vector3, b: Vector3, t: f32) -> Vector3 {
    Vector3::new(
        a.x * (1.0 - t) + b.x * t,
        a.y * (1.0 - t) + b.y * t,
        a.z * (1.0 - t) + b.z * t,
    )
}
