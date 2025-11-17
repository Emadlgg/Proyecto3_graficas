
# 📦 Assets del Proyecto

Este directorio contiene todos los recursos 3D utilizados en la simulación del Sistema Solar.

---

## 📁 Estructura

```
assets/
└── models/
    ├── sphere.obj          # Modelo de esfera para planetas
    └── spaceship.obj       # Modelo de nave espacial
```

---

## 🌐 Modelos 3D

### **sphere.obj**
- **Uso**: Renderizado de todos los planetas, sol y luna
- **Vértices**: ~480-960 (dependiendo de la versión)
- **Formato**: Wavefront OBJ
- **Características**: 
  - Geometría esférica subdividida
  - Normales calculadas
  - UVs para mapeo esférico
- **Aplicación**: Se reutiliza para todos los cuerpos celestes con diferentes shaders

### **spaceship.obj**
- **Uso**: Nave espacial del jugador
- **Vértices**: ~7,392
- **Formato**: Wavefront OBJ
- **Características**:
  - Modelo detallado de nave
  - Geometría optimizada para rendering en tiempo real
  - Escalado a 0.05 para proporciones correctas
- **Posición**: Siempre visible en la parte inferior central de la pantalla

---

## 🎨 Shaders Aplicados

Aunque los modelos son simples (esfera), los shaders procedurales crean la apariencia única de cada planeta:

| Planeta | Modelo Base | Shader | Capas |
|---------|-------------|--------|-------|
| Sol ☀️ | sphere.obj | `sun` | 5 capas (manchas, erupciones, corona) |
| Mercurio | sphere.obj | `rocky_mars` | Rocoso gris |
| Venus | sphere.obj | `rocky_earth` | Atmósfera densa |
| Tierra 🌍 | sphere.obj | `rocky_earth` | 5 capas (océanos, continentes, nubes) |
| Marte 🔴 | sphere.obj | `rocky_mars` | 4 capas (cráteres, polos) |
| Júpiter 🟠 | sphere.obj | `gas_jupiter` | 4 capas (bandas, tormenta) |
| Saturno 🪐 | sphere.obj | `gas_saturn` | 4 capas (bandas sutiles) |
| Urano | sphere.obj | `ice_neptune` | Azul hielo |
| Neptuno 🔵 | sphere.obj | `ice_neptune` | Azul intenso |
| Luna 🌙 | sphere.obj | `moon` | 3 capas (cráteres, mares) |
| Nave 🚀 | spaceship.obj | `spaceship` | Metálico con especular |

---

## 🔧 Optimizaciones Aplicadas

### Sphere.obj
- **Normalización automática**: Centrado y escalado en carga
- **Reutilización**: Un solo modelo cargado en memoria
- **Shaders procedurales**: No requiere texturas, ahorro de VRAM

### Spaceship.obj
- **Escala**: 0.05x para proporciones correctas
- **Culling**: Siempre renderizada (sin frustum culling)
- **Posición fija**: Relativa a la cámara, no al mundo

---

## 📊 Estadísticas de Memoria

```
sphere.obj:     ~20-40 KB (archivo)
spaceship.obj:  ~300-500 KB (archivo)
Total Assets:   ~350-550 KB

En memoria (runtime):
sphere.obj:     ~50-80 KB (vértices + normales + UVs)
spaceship.obj:  ~300-400 KB (7392 vértices procesados)
Total RAM:      ~350-480 KB
```

---

## 🚀 Cómo Agregar Nuevos Modelos

### 1. Formato Requerido
Los modelos deben estar en formato **Wavefront OBJ** con:
- Vértices (`v`)
- Normales (`vn`)
- Coordenadas de textura (`vt`) - opcional
- Caras trianguladas (`f`)

### 2. Ejemplo de Estructura OBJ
```obj
# Vértices
v 0.0 1.0 0.0
v -0.5 -0.5 0.5
v 0.5 -0.5 0.5

# Normales
vn 0.0 1.0 0.0
vn 0.0 -1.0 0.0

# UVs (opcional)
vt 0.5 1.0
vt 0.0 0.0
vt 1.0 0.0

# Caras (v/vt/vn)
f 1/1/1 2/2/2 3/3/2
```

### 3. Cargar en el Código
```rust
let mut new_model = Model::load_from_file("assets/models/nuevo_modelo.obj")
    .expect("No se pudo cargar el modelo");
new_model.normalize_and_center(1.0); // Escalar y centrar
```

### 4. Crear Shader Personalizado
Agrega un nuevo shader en `src/shaders.rs`:
```rust
fn mi_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    // Tu lógica de shader aquí
    Color::new(255, 0, 0)
}
```

---

## 🎨 Generación de Geometría Procedural

### Anillos de Saturno
Los anillos NO son un modelo OBJ, se generan proceduralmente:

```rust
create_ring_vertices(
    inner_radius: 1.2,
    outer_radius: 1.8,
    segments: 100
)
```

Genera ~600 vértices en runtime formando un disco plano.

### Órbitas
Las órbitas también son procedurales:

```rust
create_orbit_lines(
    radius: f32,        // Radio de la órbita
    segments: u32,      // Número de segmentos (80-200)
    inclination: f32    // Inclinación orbital
)
```

---

## 📝 Licencias y Atribuciones

### sphere.obj
- **Fuente**: Modelo estándar generado/libre
- **Licencia**: Uso libre para proyectos educativos
- **Modificaciones**: Ninguna

### spaceship.obj
- **Fuente**: Modelo estándar generado/libre
- **Licencia**: Uso libre para proyectos educativos
- **Modificaciones**: Ninguna



---

## 🔍 Verificación de Assets

Para verificar que los assets están correctamente ubicados:

```bash
# Linux/Mac
ls -lh assets/models/

# Windows
dir assets\models\
```

Deberías ver:
```
sphere.obj      (~20-40 KB)
spaceship.obj   (~300-500 KB)
```

---

## 🛠️ Troubleshooting

### "No se pudo cargar sphere.obj"
- ✅ Verifica que el archivo existe en `assets/models/sphere.obj`
- ✅ Ejecuta desde la raíz del proyecto: `cargo run --release`
- ✅ El path es relativo al directorio de ejecución

### "No se pudo cargar spaceship.obj"
- ✅ Verifica que el modelo esté en formato OBJ válido
- ✅ Asegúrate de que las caras estén trianguladas
- ✅ Verifica que tenga normales (`vn`)

### Performance bajo con spaceship.obj
- ✅ Reduce `spaceship.scale` para hacer el modelo más pequeño
- ✅ Considera usar un modelo más simple (menos vértices)
- ✅ Activa culling si el modelo tiene muchos triángulos



**Última actualización**: Noviembre 2024
