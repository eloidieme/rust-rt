# Rust Ray Tracer

A physically-based ray tracer written in Rust, based on the Ray Tracing in One Weekend series.

## Features

- **Materials**: Lambertian (diffuse), Metal (reflective), and Dielectric (glass) materials
- **Camera Controls**: Configurable field of view, depth of field, and camera positioning
- **Multiple Backgrounds**: Solid colors, vertical/horizontal gradients, and bilinear gradients
- **Scene Files**: Define scenes using YAML configuration files
- **Parallel Rendering**: Multi-threaded rendering with progress bars via Rayon
- **Output Formats**: PNG and PPM image formats

## Building

```bash
cargo build --release
```

## Usage

### Render with a scene file

```bash
cargo run --release -- --scene scenes/default.yaml -o renders/output.png
```

### Render random scene

```bash
cargo run --release -- -w 1920 -s 500 -d 50 -o renders/random.png
```

### Command-line Options

- `-w, --width <WIDTH>`: Image width in pixels (default: 1200, ignored if scene file is used)
- `-s, --samples <SAMPLES>`: Number of random samples per pixel (default: 100)
- `-d, --depth <DEPTH>`: Maximum number of ray bounces (default: 50)
- `--scene <PATH>`: Path to a scene YAML file
- `-o, --output <PATH>`: Output filename (default: renders/image.png)

## Scene File Format

Scene files are written in YAML. See the `scenes/` directory for examples.

```yaml
width: 800
aspect_ratio: 1.777
camera:
  look_from: { x: 13.0, y: 2.0, z: 3.0 }
  look_at: { x: 0.0, y: 0.0, z: 0.0 }
  vup: { x: 0.0, y: 1.0, z: 0.0 }
  fov: 20.0
  defocus_angle: 0.6
  focus_dist: 10.0
background:
  type: VerticalGradient
  top: { x: 0.5, y: 0.7, z: 1.0 }
  bottom: { x: 1.0, y: 1.0, z: 1.0 }
objects:
  - type: Sphere
    center: { x: 0.0, y: 0.0, z: -1.0 }
    radius: 0.5
    material:
      type: Lambertian
      albedo: { x: 0.8, y: 0.3, z: 0.3 }
```

### Material Types

- **Lambertian** (Diffuse): `type: Lambertian`, `albedo: { x, y, z }`
- **Metal** (Reflective): `type: Metal`, `albedo: { x, y, z }`, `fuzz: 0.0-1.0`
- **Dielectric** (Glass): `type: Dielectric`, `index: 1.5`

### Background Types

- **Solid**: `type: Solid`, `color: { x, y, z }`
- **VerticalGradient**: `type: VerticalGradient`, `top: { x, y, z }`, `bottom: { x, y, z }`
- **HorizontalGradient**: `type: HorizontalGradient`, `left: { x, y, z }`, `right: { x, y, z }`
- **BilinearGradient**: `type: BilinearGradient`, `top_left`, `top_right`, `bottom_left`, `bottom_right`

## Project Structure

```
src/
  main.rs              # CLI and entry point
  lib.rs               # Module exports
  engine.rs            # Scene loading and rendering orchestration
  scene/
    mod.rs             # Scene module
    config.rs          # Scene file parsing (YAML deserialization)
    generators.rs      # Procedural scene generators
  geometry/
    mod.rs             # Geometry module
    hittable.rs        # Ray-object intersection trait
    hittable_list.rs   # Collection of hittable objects
    sphere.rs          # Sphere primitive
  imaging/
    mod.rs             # Imaging module
    camera.rs          # Camera with DoF
    canvas.rs          # Image buffer
    color.rs           # Color utilities
    material.rs        # Material definitions
    renderer.rs        # Ray tracing algorithm
  math/
    mod.rs             # Math module
    interval.rs        # Interval arithmetic
    ray.rs             # Ray definition
    vec3.rs            # 3D vector math
    utils.rs           # Random number generation
```

## Performance Tips

- Use `--release` builds for production renders (10-20x faster)
- Adjust `-s` (samples) for quality/speed tradeoff
- Lower `-d` (depth) reduces computation for scenes without much reflection/refraction
- Use multiple CPU cores automatically via Rayon

## Examples

See the `scenes/` directory for example scene files:

- `default.yaml` - Three glass spheres
- `simple.yaml` - Basic material showcase
- `cornell_box.yaml` - Cornell box (walls and spheres)
- `sunset.yaml` - Demonstration of gradient backgrounds

## License

MIT
