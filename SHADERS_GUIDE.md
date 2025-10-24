# Lab 5 - Celestial Bodies Shaders Guide

## Overview
This project implements custom fragment shaders for rendering three different types of celestial bodies: a star (Sun), a rocky planet (Earth), and a gas giant (Jupiter). Each shader creates unique visual effects appropriate for the celestial body type.

## Implemented Shaders

### 1. Sun Shader (Star)
**File**: `src/shaders.rs` - `sun_shader()`

**Visual Features**:
- **Radial Gradient**: Color transitions from bright white-yellow core to orange-red edges
- **Pulsating Effect**: Dynamic brightness variation using depth-based sine wave
- **Color Zones**:
  - Core (0-30% radius): Bright white-yellow (#FFFEE6)
  - Middle (30-70% radius): Yellow-orange gradient
  - Edge (70-100% radius): Orange-red (#FF8019)

**Technical Implementation**:
- Calculates distance from center point for radial effect
- Uses sine function for pulsating animation
- Color interpolation based on normalized distance

### 2. Earth Shader (Rocky Planet)
**File**: `src/shaders.rs` - `earth_shader()`

**Visual Features**:
- **Procedural Continents**: Multiple noise octaves create landmass patterns
- **Diverse Terrain Types**:
  - Deep Ocean: Dark blue (#001A4D)
  - Shallow Ocean: Light blue (#1A4D99)
  - Beaches: Sandy beige (#CCB266)
  - Grasslands: Green (#33804D)
  - Forests: Dark green (#1A4D1A)
  - Mountains: Brown (#7F6650)
  - Ice Caps: White-blue (#E6E6FF)
- **Polar Ice Caps**: Latitude-based ice coverage at poles

**Technical Implementation**:
- Three noise functions with different frequencies
- Combined noise determines terrain type
- Latitude calculation for polar regions
- Threshold-based terrain classification

### 3. Gas Giant Shader (Jupiter)
**File**: `src/shaders.rs` - `gas_giant_shader()`

**Visual Features**:
- **Horizontal Bands**: Characteristic striped appearance
- **Turbulent Patterns**: Swirls and atmospheric disturbances
- **Great Red Spot**: Distinctive storm feature at specific location
- **Color Palette**:
  - Light Cream: #F2CCB3
  - Light Brown: #CC7F4D
  - Dark Brown: #994D19
  - Red Spot: #B34D33

**Technical Implementation**:
- Sine wave based band positioning
- Turbulence using position-based sine/cosine
- Distance calculation for storm feature
- Detail noise for atmospheric texture

## How to Use

### Running the Application

```bash
# Build the project
cargo build

# Run the application
cargo run
```

### Controls

**Model Switching**:
- `SPACE`: Manually switch between celestial bodies
- Auto-switch: Every 5 seconds

**Camera/Transform Controls**:
- `Arrow Keys`: Move the model (Up/Down/Left/Right)
- `S/A`: Scale up/down
- `Q/W`: Rotate around X-axis
- `E/R`: Rotate around Y-axis
- `T/Y`: Rotate around Z-axis

**General**:
- `ESC` or Close Window: Exit application

### Switching Between Models

The application cycles through three models:
1. **Sun** (`13913_Sun_v2_l3.obj`) - Uses Sun shader
2. **Earth** (`13902_Earth_v1_l3.obj`) - Uses Earth shader
3. **Jupiter** (`13905_Jupiter_V1_l3.obj`) - Uses Gas Giant shader

**Note**: The model files use inconsistent naming conventions (V1 vs v1/v2). This is inherited from the source models and doesn't affect functionality.

## Customization

### Modifying Shader Colors

Edit the color definitions in `src/shaders.rs`:

```rust
// Example: Change Earth's ocean color
let deep_ocean = Vector3::new(0.0, 0.1, 0.3);  // R, G, B (0.0-1.0)
```

### Adding New Celestial Bodies

1. Add the model file to `assets/models/`
2. Create a new shader function in `src/shaders.rs`
3. Add a new `ShaderType` variant in `src/main.rs`
4. Update the `celestial_bodies` vector in `main()` function
5. Update the match statement in `render()` function

Example for adding Mars:

```rust
// Step 1: In src/shaders.rs - Add new shader function
pub fn mars_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(0.8, 0.3, 0.2)  // Reddish color
}

// Step 2: In src/main.rs - Add to ShaderType enum (around line 27)
pub enum ShaderType {
    Sun,
    Earth,
    Mars,  // New
    GasGiant,
    Default,
}

// Step 3: In src/main.rs - Update celestial_bodies vector (around line 140)
let celestial_bodies = vec![
    ("assets/models/13913_Sun_v2_l3.obj", ShaderType::Sun, "Sun"),
    ("assets/models/13902_Earth_v1_l3.obj", ShaderType::Earth, "Earth"),
    ("assets/models/mars.obj", ShaderType::Mars, "Mars"),  // New
    ("assets/models/13905_Jupiter_V1_l3.obj", ShaderType::GasGiant, "Jupiter"),
];

// Step 4: In src/main.rs - Update imports (around line 16)
use shaders::{vertex_shader, sun_shader, earth_shader, mars_shader, gas_giant_shader, default_shader};

// Step 5: In src/main.rs - Update match in render() (around line 109)
let final_color = match uniforms.shader_type {
    ShaderType::Sun => sun_shader(&fragment, uniforms),
    ShaderType::Earth => earth_shader(&fragment, uniforms),
    ShaderType::Mars => mars_shader(&fragment, uniforms),  // New
    ShaderType::GasGiant => gas_giant_shader(&fragment, uniforms),
    ShaderType::Default => default_shader(&fragment),
};
```

### Adjusting Animation Speed

Modify the rotation speed in `main()`:

```rust
// Auto-rotate for visual effect
rotation.y += 0.01;  // Increase for faster rotation
```

## Technical Details

### Shader Pipeline

1. **Vertex Shader** (`vertex_shader()`):
   - Transforms vertex positions using model matrix
   - Converts to homogeneous coordinates
   - Performs perspective division

2. **Rasterization** (`triangle()`):
   - Converts triangles to fragments (pixels)
   - Uses Bresenham's line algorithm

3. **Fragment Shader** (Various):
   - Computes final pixel color
   - Applies procedural effects
   - Returns RGB color vector

### Performance Considerations

- **Shader Complexity**: More complex shaders (Earth) may be slower
- **Model Complexity**: Higher polygon count affects rendering speed
- **Optimization**: Profile is set to `opt-level = 3` in debug mode
- **Frame Rate**: Target is 60 FPS (16ms per frame)

## Troubleshooting

### Black Screen Issues

If you see a black screen:
1. Verify model files exist in `assets/models/`
2. Check console output for error messages
3. Ensure shader functions return non-zero colors
4. Verify framebuffer is initialized properly

### Model Not Loading

```
Error: Failed to load obj
```

**Solution**: Check the file path in `main.rs` around line 152 in the `celestial_bodies` vector. Verify:
- The file exists in `assets/models/` directory
- The filename matches exactly (including case sensitivity)
- The path string is correct

### Performance Issues

- Reduce model complexity
- Simplify shader calculations
- Run with release mode: `cargo run --release`

## Lab Requirements Met

✅ **Star Shader (Sun)**: Implemented with radial gradient and pulsating effect  
✅ **Rocky Planet Shader (Earth)**: Procedural continents with varied terrain types  
✅ **Gas Giant Shader (Jupiter)**: Horizontal bands with storm features  

## Additional Resources

- [Raylib Documentation](https://www.raylib.com/)
- [Procedural Generation Techniques](https://en.wikipedia.org/wiki/Procedural_generation)
- [Fragment Shaders](https://en.wikipedia.org/wiki/Shader#Fragment_shaders)

## Credits

Lab 5 - Computer Graphics Course  
Shader implementations for celestial body visualization
