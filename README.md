# Lab 5 - Sistema Solar Procedural

**Renderer 3D con Shaders Procedurales Creativos** - Implementación de un sistema solar miniatura usando técnicas de programación gráfica sin texturas, con 6 planetas y shaders multi-capa.

## 🌍 Descripción

Un renderer que renderiza **simultáneamente 6 cuerpos celestes** en órbita alrededor del **Sol**, todo generado con shaders procedurales creativos (sin texturas ni materiales). Cada planeta tiene un shader único con 5+ capas de complejidad.

## 📋 Planetas Implementados

```
                    ☆ Sol (Centro) - 5 capas
                   /|\
                  / | \
                 /  |  \
                /   |   \
            ◉ Venus   ◉ Tierra - 5 capas + Luna
         (Órbita      (Órbita
          cercana)     media)
                    
    ◉ Gigante Gaseoso - 5 capas + Anillos (Órbita lejana)
    ◉ Urano - 5 capas (Órbita muy lejana)
    ◉ Neptuno - 5 capas (Órbita extrema)
```

### 🌟 Shaders Creativos (5+ Capas cada uno)

| Planeta | Tipo | Capas | Características Creativos |
|---------|------|-------|---------------------------|
| **Sol** | 0 | 5 | Gradiente de núcleo, fotosfera turbulenta, prominencias solares, corona luminosa, líneas magnéticas |
| **Tierra** | 1 | 5 | Océanos, continentes complejos, cordilleras, nubes animadas, casquetes polares + atmósfera |
| **Gigante Gaseoso** | 2 | 5 | Atmósfera naranja, bandas horizontales, tormentas turbulentas, Gran Mancha Roja, relámpagos |
| **Venus** | 7 | 5 | Atmósfera amarilla densa, remolinos de nubes tóxicas, puntos volcánicos ardientes, bandas de super-rotación, resplandor de invernadero |
| **Neptuno** | 5 | 5 | Base oceánica azul profunda, bandas de metano, Gran Mancha Oscura, vientos blancos de alta altitud, turbulencia atmosférica |
| **Urano** | 6 | 5 | Hielo cianita, patrones de escarcha, bandas polares sutiles, tormenta inclinada (rotación de lado), brillo de hielo |
| **Luna (Tierra)** | 3 | 4 | Crateres, sombras, picos brillantes, variaciones de color |
| **Anillos (Gas Giant)** | 4 | 4 | Bandas de anillos, partículas, efecto de profundidad, oscurecimiento de bordes |

## 📋 Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Project Structure](#project-structure)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building](#building)
- [Usage](#usage)
- [Technical Details](#technical-details)
- [Architecture](#architecture)
- [Dependencies](#dependencies)
- [Contributing](#contributing)
- [License](#license)

## 🎯 Disposición del Sistema Solar

```
            ☆ Sol (Centro)
           /|\
          / | \
         /  |  \
        /   |   \
       /    |    \
      /     |     \
  ◉ Venus  ◉ Tierra  ◉ Gigante Gaseoso
 (Órbita   (Órbita    (Órbita lejana)
  cercana) media)
      
      ◉ Urano + ◉ Neptuno (Órbitas extremas)
```

**En pantalla:**
- **Sol**: Fijo en el centro, rotando sobre su eje
- **Tierra**: Orbita cercana alrededor del Sol (radio: 90px) + Luna satélite
- **Venus**: Órbita muy cercana (radio: 65px)
- **Gigante Gaseoso**: Órbita lejana alrededor del Sol (radio: 140px) + Anillos
- **Urano**: Órbita muy lejana (radio: 160px)
- **Neptuno**: Órbita extrema (radio: 180px)

This project is an educational graphics renderer that demonstrates core computer graphics concepts in Rust. It provides a complete pipeline for loading 3D models (OBJ format), applying transformations, and rendering them in real-time using a custom rasterization approach combined with Raylib for window management.

**Key Focus Areas:**
- Matrix mathematics and transformations
- OBJ file format parsing and mesh loading
- Vertex and fragment shading concepts
- Framebuffer management and pixel manipulation
- Real-time rendering loop and performance optimization

## ✨ Features

- **6 Cuerpos Celestes Creativos**: Sol, Tierra, Venus, Gigante Gaseoso, Urano y Neptuno
- **Shaders Procedurales Multi-capa (5+ capas)**: Cada planeta tiene un shader único y creativo sin usar texturas
- **Sistema de Lunas**: La Tierra posee una Luna satélite que orbita alrededor de ella
- **Sistema de Anillos**: El Gigante Gaseoso tiene anillos procedurales planos
- **Mecánica Orbital 3D**: Órbitas elípticas con inclinación, velocidades independientes
- **Rotación y Traslación**: Cada planeta rota sobre su eje y se traslada en su órbita
- **Animación en Tiempo Real**: Shaders animados con patrones dinámicos (tiempo)
- **Matriz Transformaciones Completa**: Traslación, rotación y escalado en 3D
- **Framebuffer Personalizado**: Gestión de píxeles con rasterización de triángulos
- **Cargador OBJ**: Carga modelos 3D en formato Wavefront OBJ
- **Controles Interactivos**: Cámara, zoom, rotación del sistema, pausa

## 📁 Project Structure

```
Lab5/
├── Cargo.toml                 # Rust project manifest with dependencies
├── README.md                  # This comprehensive guide
├── CONTROLES.md              # Control keys and shortcuts
├── .gitattributes            # Git attributes configuration
├── src/                       # Rust source code
│   ├── main.rs              # Main application with 6 celestial bodies
│   ├── shaders.rs           # 8 Shaders procedurales creativos (5+ capas)
│   │                           ├── sun_shader (5 capas)
│   │                           ├── earth_shader (5 capas)
│   │                           ├── gas_giant_shader (5 capas)
│   │                           ├── venus_shader (5 capas)
│   │                           ├── neptune_shader (5 capas)
│   │                           ├── uranus_shader (5 capas)
│   │                           ├── moon_shader (4 capas)
│   │                           └── ring_shader (4 capas)
│   ├── framebuffer.rs       # Framebuffer management and rendering
│   ├── triangle.rs          # Barycentric coordinate rasterization
│   ├── obj.rs               # OBJ file loader and model parser
│   ├── vertex.rs            # Vertex structure and attributes
│   ├── fragment.rs          # Fragment/pixel shader implementation
│   ├── line.rs              # Line drawing algorithm
│   ├── matrix.rs            # Matrix mathematics and transformations
│   ├── rings.rs             # Procedural ring generation
│   └── moons.rs             # Moon system scaffolding
├── assets/
│   └── models/              # 3D model files
│       ├── 13902_Earth_v1_l3.obj/mtl
│       ├── 13905_Jupiter_V1_l3.obj/mtl
│       ├── 13907_Uranus_v2_l3.obj/mtl
│       ├── 13913_Sun_v2_l3.obj/mtl
│       └── 10464_Asteroid_v1_Iterations-2.obj/mtl
└── target/                  # Build artifacts (generated)
```

## 📋 Prerequisites

Before building this project, ensure you have the following installed:

- **Rust**: 1.70 or later ([Install Rust](https://rustup.rs/))
- **Cargo**: Comes with Rust installation
- **C++ Compiler**: Required by Raylib dependency
  - **Linux**: `sudo apt-get install build-essential`
  - **macOS**: Xcode Command Line Tools (`xcode-select --install`)
  - **Windows**: Visual Studio Build Tools or MinGW

## 🚀 Installation

### Clone the Repository

```bash
git clone https://github.com/Kosho969/graphics.git
cd graphics
git checkout SR-Models
```

### Verify Rust Installation

```bash
rustc --version
cargo --version
```

## 🔨 Building

### Debug Build (Development)

```bash
cargo build
```

This creates optimized debug binaries with debug symbols for faster compilation and easier debugging.

### Release Build (Production)

```bash
cargo build --release
```

This creates highly optimized binaries for better runtime performance.

## ▶️ Usage

### Running the Application

```bash
cargo run --release
```

### In-Application Controls

The solar system is fully interactive. While the application is running, use these controls:

**Camera Movement:**
- Arrow keys (↑ ↓ ← →) - Pan the camera
- **S** - Zoom in (up to 3x)
- **A** - Zoom out (down to 0.3x)

**System Rotation:**
- **Q/W** - Rotate around X axis
- **E/R** - Rotate around Y axis  
- **T/Y** - Rotate around Z axis

**Animation Control:**
- **SPACE** - Pause/Resume planet rotations
- **O** - Pause/Resume orbital motion

For a complete guide, see [CONTROLES.md](./CONTROLES.md)

### Dynamic Features

The system automatically:
- Renders 6 celestial bodies simultaneously
- Calculates orbital mechanics in real-time with 3D inclination
- Generates procedural textures (5+ layer shaders) for each planet
- Updates FPS counter and timing information
- Displays all available controls on screen
- Renders lunas and anillos for specific planets

## 🧮 Technical Details

### 🎨 Shader Creativity & Complexity Analysis

Each shader implements **5+ layers** of procedural generation using advanced noise functions:

#### **Sol (5 Capas) - Realistic Star Surface**
1. **Core Gradient**: Temperature gradient (blanco → amarillo → naranja)
2. **Photosphere**: Turbulencia FBM de 4 octavas para superficie realista
3. **Solar Prominences**: Destellos brillantes animados por tiempo
4. **Corona Glow**: Efecto de atmósfera de plasma alrededor
5. **Magnetic Fields**: Patrones de líneas magnéticas fluyen dinámicamente

**Técnicas**: FBM (Fractional Brownian Motion), smoothstep blending, animación temporal

#### **Tierra (5 Capas) - Realistic Earth**
1. **Ocean Base**: Azul profundo como océanos reales
2. **Landmasses**: Ruido de 5 octavas para forma de continentes complejos
3. **Mountain Ranges**: Picos brillantes en cordilleras
4. **Animated Clouds**: Remolinos de nubes en 2 niveles con offset temporal
5. **Polar Ice Caps + Atmosphere**: Casquetes de hielo brillantes + brillo atmosférico en bordes

**Técnicas**: Ruido multi-octava, animación de nubes con tiempo, mezcla de colores

#### **Gigante Gaseoso (5 Capas) - Complex Storm System**
1. **Base Atmosphere**: Gradiente naranja-marrón (base gaseosa)
2. **Atmospheric Bands**: Bandas horizontales oscilantes con animación temporal
3. **Turbulent Storms**: Tormenta a gran escala generada por FBM doble
4. **Great Red Spot**: Mancha roja gigante con interior turbulento (simulación realista)
5. **Lightning & Disturbances**: Relámpagos dinámicos y disturbios atmosféricos

**Técnicas**: FBM dual, efecto de mancha focal, animación rápida de rayos

#### **Venus (5 Capas) - Hellish Atmosphere**
1. **Base Yellow Atmosphere**: Atmósfera amarilla hellish
2. **Thick Toxic Clouds**: Remolinos de nubes tóxicas con doble FBM
3. **Volcanic Hot Spots**: Puntos volcánicos ardientes (rojo-naranja)
4. **Super-Rotation Bands**: Bandas de super-rotación rápida (característica de Venus)
5. **Greenhouse Glow**: Resplandor de efecto invernadero en los bordes

**Técnicas**: Animación rápida, patrones complejos, efecto de borde

#### **Neptuno (5 Capas) - Deep Ocean Ice Giant**
1. **Deep Ocean Base**: Azul marino profundo realista
2. **Methane Cloud Bands**: Bandas de nubes de metano
3. **Great Dark Spot**: Tormenta gigante similar a la de Júpiter
4. **High-Altitude White Streaks**: Vientos de alta velocidad en forma de rayas blancas
5. **Atmospheric Turbulence**: Turbulencia atmosférica compleja

**Técnicas**: FBM de 4 octavas, animación de vientos, mancha focal

#### **Urano (5 Capas) - Tilted Ice Giant**
1. **Icy Cyan Base**: Base de hielo cianita
2. **Methane Frost Patterns**: Patrones de escarcha de metano
3. **Subtle Polar Bands**: Bandas polares débiles (único entre gigantes)
4. **Tilted Storm Spot**: Tormenta inclinada (Urano rota de lado)
5. **Icy Gloss & Shimmer**: Brillo y destello de hielo

**Técnicas**: Inclinación de parámetros, animación de tormenta, efecto de brillo

### Rendering Pipeline

The project implements a complete graphics pipeline with the following stages:

1. **Vertex Processing**
   - Load vertices from OBJ files
   - Apply model-view-projection transformations
   - Output transformed vertices

2. **Rasterization**
   - Convert triangles to screen-space pixels
   - Barycentric coordinate rasterization
   - Depth interpolation

3. **Fragment Processing**
   - Compute final pixel color using procedural shaders
   - Apply planet-specific shader effects (5+ layers per planet)
   - Write to framebuffer

4. **Output**
   - Display framebuffer contents using Raylib
   - Handle window management and events

### Key Components

#### Matrix Module (`matrix.rs`)
- 4x4 matrix implementation
- Matrix multiplication
- Transformation matrices (translation, rotation, scaling)
- Vector-matrix operations

#### Vertex Shader (`shaders.rs`)
- Transforms vertex positions using model matrix
- Computes vertex attributes for fragment shader

#### Fragment Shader (`fragment.rs`)
- Computes final pixel color
- Implements lighting models and color interpolation
- Outputs to framebuffer

#### Framebuffer (`framebuffer.rs`)
- Manages pixel buffer (2D array of colors)
- Provides pixel write operations
- Handles framebuffer clearing and swapping

#### OBJ Loader (`obj.rs`)
- Parses OBJ file format
- Loads vertex positions and normals
- Handles material references
- Uses `tobj` crate for parsing

### Coordinate System

The project uses a standard 3D coordinate system:
- **X-axis**: Right
- **Y-axis**: Up
- **Z-axis**: Toward viewer (right-handed coordinate system)

## 🏗️ Architecture

### Data Flow

```
OBJ File
    ↓
[OBJ Loader] → Vertices & Indices
    ↓
[Vertex Shader] → Transformed Vertices
    ↓
[Rasterizer] → Screen-space Primitives
    ↓
[Fragment Shader] → Pixel Colors
    ↓
[Framebuffer] → Pixel Buffer
    ↓
[Raylib] → Screen Display
```

### Main Loop Structure

```rust
loop {
    // 1. Update uniforms and transformations
    let uniforms = create_model_matrix(...);
    
    // 2. Clear framebuffer
    framebuffer.clear();
    
    // 3. Render geometry
    for triangle in model.triangles {
        render_triangle(&mut framebuffer, triangle, &uniforms);
    }
    
    // 4. Swap buffers and display
    framebuffer.swap_buffers(&draw_handle);
}
```

## 📦 Dependencies

| Dependency | Version | Purpose |
|-----------|---------|---------|
| **raylib** | 5.5.1 | Window management, graphics context, event handling |
| **tobj** | 4.0.2 | OBJ file format parsing and loading |

These are specified in `Cargo.toml`:

```toml
[dependencies]
raylib = "5.5.1"
tobj = "4.0.2"
```

## 🎓 Learning Resources

This project demonstrates several important computer graphics concepts:

1. **Matrix Mathematics**
   - Homogeneous coordinates
   - Affine transformations
   - Coordinate system transformations

2. **3D Graphics Pipeline**
   - Vertex processing
   - Primitive assembly
   - Rasterization
   - Fragment processing
   - Output merging

3. **OBJ File Format**
   - Vertex data structure
   - Face definitions
   - Material references
   - Normal vectors

4. **Rendering Concepts**
   - Framebuffer operations
   - Double buffering
   - Per-pixel operations
   - Color representation

## 🐛 Troubleshooting

### Build Fails with "Raylib not found"

**Solution**: Ensure you have a C++ compiler installed:
```bash
# Linux
sudo apt-get install build-essential

# macOS
xcode-select --install
```

### Application runs but displays black screen

**Possible causes**:
- Model file path is incorrect
- Framebuffer is not being updated
- Transformation matrices are incorrect

**Solutions**:
- Verify model file exists in `assets/models/`
- Check the shader implementations
- Review the transformation calculations

### Performance Issues

- Use release build: `cargo run --release`
- Reduce model complexity
- Profile with: `cargo flamegraph --release`

## 🤝 Contributing

Contributions are welcome! If you find issues or want to add features:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is part of a computer graphics course. Please refer to the repository's LICENSE file for details.

## 🔗 Repository Information

- **Repository**: [Kosho969/graphics](https://github.com/Kosho969/graphics)
- **Branch**: SR-Models
- **Type**: Educational Graphics Renderer

## 📚 Additional References

- [Raylib Documentation](https://www.raylib.com/)
- [Wavefront OBJ Format](https://en.wikipedia.org/wiki/Wavefront_.obj_file)
- [Computer Graphics: Principles and Practice](https://www.elsevier.com/books/computer-graphics/foley/978-0-321-39952-6)
- [Rust Graphics Programming](https://www.rust-lang.org/)

---

**Last Updated**: October 2025  
**Project Version**: 0.1.0
