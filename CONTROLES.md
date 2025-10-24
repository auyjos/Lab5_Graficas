# Controles del Renderizador 3D

## Controles Básicos

### Movimiento Manual del Planeta
- **Flechas Izquierda/Derecha**: Mover el planeta en el eje X
- **Flechas Arriba/Abajo**: Mover el planeta en el eje Y

### Escala
- **S**: Aumentar el tamaño del planeta
- **A**: Disminuir el tamaño del planeta

### Rotación Manual
- **Q/W**: Rotar alrededor del eje X (adelante/atrás)
- **E/R**: Rotar alrededor del eje Y (arriba/abajo)
- **T/Y**: Rotar alrededor del eje Z (lado a lado)

### Animación Automática
- **ESPACIO**: Activar/Desactivar rotación automática del planeta
- **O**: Activar/Desactivar órbita automática alrededor del centro

## Características Implementadas

### Rotación Automática
El planeta rota continuamente alrededor del eje Y cuando la rotación automática está activada.
- Velocidad de rotación: 0.02 radianes por frame
- Puede ser pausada/reanudada con la tecla ESPACIO

### Órbita Automática
El planeta orbita alrededor del punto central (400, 300) cuando la órbita automática está activada.
- Radio de órbita: 100 píxeles
- Velocidad de órbita: 0.5 radianes por segundo
- Puede ser pausada/reanudada con la tecla O

### Transformaciones de Matriz
Las transformaciones se aplican en el siguiente orden:
1. **Escala**: El modelo se escala a un tamaño específico
2. **Rotación**: Se aplican rotaciones en X, Y, Z
3. **Traslación**: El objeto se posiciona en la pantalla

## Parámetros Ajustables

En el código (`main.rs`), puedes ajustar:

```rust
let orbit_radius = 100.0;          // Radio de la órbita (en píxeles)
let orbit_speed = 0.5;             // Velocidad de órbita (radianes/segundo)
let rotation_speed = 0.02;         // Velocidad de rotación (radianes/frame)
```

## Ejemplo de Uso

1. Ejecuta el programa: `cargo run --release`
2. El planeta aparecerá girando automáticamente
3. Presiona ESPACIO para pausar la rotación
4. Presiona O para pausar la órbita
5. Usa flechas para mover manualmente
6. Usa S/A para aumentar/disminuir el tamaño
7. Usa Q/W/E/R/T/Y para rotación manual
