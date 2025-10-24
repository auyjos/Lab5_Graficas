# Sistema Solar - Guía de Controles Completa

## 🎮 Controles Disponibles

### Movimiento de Cámara
- **Flechas (↑ ↓ ← →)**: Mover la cámara en los ejes X e Y
  - **Arriba (↑)**: Mover cámara hacia arriba
  - **Abajo (↓)**: Mover cámara hacia abajo
  - **Izquierda (←)**: Mover cámara hacia la izquierda
  - **Derecha (→)**: Mover cámara hacia la derecha

### Zoom
- **S**: Aumentar zoom (acercar)
- **A**: Disminuir zoom (alejar)
  - El zoom está limitado entre 0.3x y 3.0x

### Rotación del Sistema Solar
- **Q**: Rotar el sistema alrededor del eje X (hacia atrás)
- **W**: Rotar el sistema alrededor del eje X (hacia adelante)
- **E**: Rotar el sistema alrededor del eje Y (hacia la izquierda)
- **R**: Rotar el sistema alrededor del eje Y (hacia la derecha)
- **T**: Rotar el sistema alrededor del eje Z (contrareloj)
- **Y**: Rotar el sistema alrededor del eje Z (reloj)

### Animación
- **SPACE**: Pausar/Reanudar la rotación de los planetas sobre su eje
- **O**: Pausar/Reanudar las órbitas de los planetas alrededor del sol

---

## 🌍 Sistema Solar

---

## 🌍 Sistema Solar

El programa renderiza simultáneamente **3 cuerpos celestes**:

### 1. **Sol (Amarillo/Dorado)**
- Posición: Centro (400, 300, 0)
- Escala: 40 unidades
- Rotación: Contínua (cuando Auto Rotate está activo)
- Órbita: Ninguna (fijo en el centro)
- Shader: Turbulencias, coronas y destellos solares

### 2. **Tierra (Azul)**
- Posición: Órbita cercana (radio: 80 unidades)
- Escala: 25 unidades
- Rotación: Contínua (cuando Auto Rotate está activo)
- Órbita: Inclinada en el plano X-Y (cuando Auto Orbit está activo)
- Shader: Océanos azules, continentes, nubes, hielo
- Velocidad de órbita: 0.03 rad/s (rápida)

### 3. **Gigante Gaseoso (Naranja/Marrón)**
- Posición: Órbita lejana (radio: 130 unidades)
- Escala: 35 unidades
- Rotación: Contínua (cuando Auto Rotate está activo)
- Órbita: Inclinada en plano diferente (cuando Auto Orbit está activo)
- Shader: Bandas de color, tormentas, grandes manchas rojas
- Velocidad de órbita: 0.015 rad/s (lenta)

---

## 🎨 Características de Renderizado

### Shaders Procedurales (Sin Texturas)
Cada cuerpo celeste tiene su propio shader procedural generado en tiempo real:

**Sol:**
- Capa 1: Color base dorado
- Capa 2: Turbulencia de superficie (FBM)
- Capa 3: Efecto de corona animado
- Capa 4: Destellos solares dinámicos

**Tierra:**
- Capa 1: Océano azul profundo
- Capa 2: Continentes verdes con ruido Perlin
- Capa 3: Nubes blancas animadas
- Capa 4: Casquetes de hielo polares

**Gigante Gaseoso:**
- Capa 1: Color base naranja
- Capa 2: Bandas horizontales
- Capa 3: Tormentas y patrones turbulecos
- Capa 4: Grandes manchas rojas

### Órbitas Inclinadas 3D
- Cada planeta orbita en un plano inclinado diferente
- La inclinación se basa en el tipo de planeta
- Esto crea una visualización más realista de un sistema solar

### Renderizado en Tiempo Real
- FPS mostrado en pantalla (arriba a la izquierda)
- Tiempo transcurrido mostrado
- Renderizado triangular con rasterización optimizada
- Proyección ortográfica

---

## 💡 Consejos de Uso

1. **Para ver mejor la órbita 3D**: Usa Q/W y E/R para rotar el sistema y ver los planetas moviéndose en el eje Z
2. **Para zoom in detallado**: Presiona S varias veces para acercarte
3. **Para pausar y observar**: Presiona SPACE para pausar la rotación de los planetas y O para pausar las órbitas
4. **Para cambiar perspectiva**: Combina los controles de rotación (Q-W-E-R-T-Y) con movimiento de cámara (flechas)
5. **Para ver toda la órbita**: Usa A para zoom out y observar todo el sistema

---

## 📊 Información en Pantalla

**Arriba (Verde):**
- FPS actual del renderer

**Centro Arriba (Blanco):**
- Título del sistema: "Sistema Solar - 3 Cuerpos Celestes"

**Arriba Izquierda (Gris):**
- Tiempo transcurrido en segundos

**Abajo (Amarillo y Gris):**
- Todos los controles disponibles
- Estado de Auto Rotate y Auto Orbit
- Nivel de zoom actual

---

## 🔧 Valores por Defecto

- Zoom: 1.0x
- Camera Offset: (0, 0, 0)
- System Rotation: (0, 0, 0)
- Auto Rotate: **Activado**
- Auto Orbit: **Activado**
4. Casquetes polares

### GIGANTE GASEOSO (Tipo 2)
**4 capas de complejidad:**
1. Base naranja-rojiza
2. Bandas atmosféricas horizontales
3. Tormentas con patrones procedurales
4. Gran Mancha Roja destacada

## Pantalla

La pantalla muestra en tiempo real:
- **FPS**: Fotogramas por segundo (arriba izquierda, verde)
- **Título**: Sistema Solar - 3 Cuerpos Celestes (blanco)
- **Tiempo**: Contador de tiempo en segundos (gris)

## Ejecución

```bash
cargo run --release
```

¡El sistema solar comenzará a renderizarse automáticamente!

## Parámetros Ajustables

En `main.rs` puedes modificar:

```rust
CelestialBody {
    scale: 40.0,          // Tamaño del planeta
    orbit_radius: 0.0,    // Distancia al Sol
    orbit_speed: 0.0,     // Velocidad orbital
    rotation_speed: 0.02, // Velocidad de rotación
}
```

## Teclas Disponibles

| Tecla | Efecto |
|-------|--------|
| ESPACIO | Pausar/Reanudar rotaciones |
| O | Pausar/Reanudar órbitas |
| Flechas | Movimiento del sistema |
| S | Aumentar escala |
| A | Disminuir escala |
| ESC | Salir de la aplicación |

## Notas Técnicas

- La proyección es ortográfica (paralela)
- El Sol está siempre en el centro: (400, 300)
- Las órbitas son ecuatoriales (en el plano XY)
- Todos los planetas rotan alrededor del eje Y
- Los shaders son procedurales (100% código, sin texturas)


