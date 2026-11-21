# Geometrical Shapes

A Rust project that implements and draws basic geometrical shapes using the `raster` crate.

The project demonstrates Rust concepts such as structs, traits, modules, methods, and unit testing. It generates an image containing different geometrical shapes, each displayed with randomly generated colors.

## Features

The project implements five geometrical shapes:

- **Point** – Represents a single pixel.
- **Line** – Connects two points.
- **Rectangle** – Defined by two opposite points.
- **Triangle** – Defined by three points.
- **Circle** – Defined by a center point and a radius.

Additional features:

- Random shape generation.
- Random RGB color generation.
- Pixel-level drawing.
- Image boundary checking.
- Unit tests for shape initialization, random generation, and edge cases.

## Project Structure

```text
.
├── src/
│   ├── main.rs
│   └── geometrical_shapes/
│       ├── mod.rs
│       ├── traits.rs
│       ├── point.rs
│       ├── line.rs
│       ├── rectangle.rs
│       ├── triangle.rs
│       ├── circle.rs
│       └── tests.rs
├── Cargo.toml
└── README.md
```

### Architecture

The `geometrical_shapes` module contains the shape implementations and shared traits.

- `Drawable`: Defines how geometrical shapes are drawn and provides default random color generation.
- `Displayable`: Defines how individual pixels are displayed on an image.
- `tests.rs`: Contains unit tests for the geometrical shapes.

## Requirements

- Rust
- Cargo

## Installation

Clone the repository:

```bash
git clone https://learn.zone01oujda.ma/git/khammaou/drawing.git
```

Navigate into the project directory:

```bash
cd drawing
```

Build the project:

```bash
cargo build
```

## Usage

Run the application:

```bash
cargo run
```

The program creates a 1000 × 1000 image, draws geometrical shapes, and saves the result as:

```text
image.png
```

To open the generated image on Linux:

```bash
xdg-open image.png
```

## Running Tests

Run all unit tests:

```bash
cargo test
```

Run a specific test:

```bash
cargo test zero_radius_draws_one_pixel
```

Display test output, including `println!` statements:

```bash
cargo test -- --nocapture
```

## Technologies

- **Rust** – Programming language.
- **Cargo** – Package manager and build system.
- **raster** – Image creation and pixel manipulation.
- **rand** – Random number generation.

## Output

The application generates an image containing randomly colored geometrical shapes.

The output is saved locally as `image.png`.