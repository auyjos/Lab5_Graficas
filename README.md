# Lab 5 - Sistema Solar Procedural

**Renderer 3D con Shaders Procedurales** - Implementación de un sistema solar miniatura usando técnicas de programación gráfica sin texturas.

## 🌍 Descripción

Un renderer que renderiza **simultáneamente 3 cuerpos celestes** en órbita alrededor del **Sol**, todo generado con shaders procedurales (sin texturas ni materiales).

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
  ◉ Tierra◉|◉ Gas Giant
 (Órbita    |  (Órbita
  cercana)  |   lejana)
```

**En pantalla:**
- **Sol**: Fijo en el centro, rotando sobre su eje
- **Tierra**: Orbita cercana alrededor del Sol (radio: 80px)
- **Gigante Gaseoso**: Órbita lejana alrededor del Sol (radio: 130px)

This project is an educational graphics renderer that demonstrates core computer graphics concepts in Rust. It provides a complete pipeline for loading 3D models (OBJ format), applying transformations, and rendering them in real-time using a custom rasterization approach combined with Raylib for window management.

**Key Focus Areas:**
- Matrix mathematics and transformations
- OBJ file format parsing and mesh loading
- Vertex and fragment shading concepts
- Framebuffer management and pixel manipulation
- Real-time rendering loop and performance optimization

## ✨ Features

- **3D Model Loading**: Load and render OBJ format 3D models with support for materials (MTL files)
- **Matrix Transformations**: Full support for translation, rotation, and scaling transformations
- **Real-time Rendering**: Smooth 60+ FPS rendering with continuous framebuffer updates
- **Multiple Models**: Pre-loaded character and object models for demonstration
- **Custom Graphics Pipeline**: Educational implementation of vertex/fragment shading concepts
- **Framebuffer Abstraction**: Efficient pixel buffer management
- **Interactive Window**: Raylib-based window management with support for user input

## 📁 Project Structure

```
computer-graphics-v3/
├── Cargo.toml                 # Rust project manifest with dependencies
├── README.md                  # Original lesson notes
├── README_DETAILED.md         # This comprehensive guide
├── .gitattributes            # Git attributes configuration
├── src/                       # Rust source code
│   ├── main.rs              # Main application entry point and render loop
│   ├── framebuffer.rs       # Framebuffer management and rendering
│   ├── obj.rs               # OBJ file loader and model parser
│   ├── vertex.rs            # Vertex structure and attributes
│   ├── fragment.rs          # Fragment/pixel shader implementation
│   ├── shaders.rs           # Shader programs (vertex and fragment)
│   ├── triangle.rs          # Triangle rasterization
│   ├── line.rs              # Line drawing algorithm
│   ├── matrix.rs            # Matrix mathematics and transformations
│   └── vertex.rs            # Vertex data structures
├── assets/
│   └── models/              # 3D model files
│       ├── anya.obj/mtl     # Anya character model
│       ├── barbara.obj/mtl  # Barbara character model
│       ├── kumoko.obj/mtl   # Kumoko character model
│       ├── nicole.obj/mtl   # Nicole character model
│       ├── rem.obj/mtl      # Rem character model
│       ├── zelda.obj/mtl    # Zelda character model
│       ├── spaceship.obj    # Spaceship model
│       └── model.obj        # Generic model
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
cargo run
```

For release mode (faster and recommended):

```bash
cargo run --release
```

### In-Application Controls

The solar system is fully interactive. While the application is running, use these controls:

**Camera Movement:**
- Arrow keys (↑ ↓ ← →) - Pan the camera
- **S** - Zoom in
- **A** - Zoom out

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
- Renders 3 celestial bodies simultaneously
- Calculates orbital mechanics in real-time
- Generates procedural textures (shaders) for each planet
- Updates FPS counter and timing information
- Displays all available controls on screen

## 🧮 Technical Details

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
   - Apply planet-specific shader effects (4 layers per planet)
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
