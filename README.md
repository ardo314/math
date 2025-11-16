# Math WIT Library

A comprehensive WebAssembly Component Model math library providing vector, matrix, rotation, and geometric types and operations.

## Package Information

- **Package**: `ardo314:math@0.0.3`
- **Registry**: `docker.io`

## Features

This library provides WIT definitions for common mathematical operations including:

### Vector Operations
- **Vector2D, Vector3D, Vector4D**: Basic arithmetic (add, sub, mul, div), dot product, normalization, and length calculations
- **Vector3D**: Additional cross product support

### Matrix Operations
- **Matrix2x2, Matrix3x3, Matrix4x4**: Identity matrices, matrix multiplication, and vector transformation

### Rotation Representations
- **Rotation Vector**: Axis with angle magnitude representation
- **Axis-Angle**: Separate axis and angle representation
- **Quaternion**: Quaternion-based rotation
- **Rotation Matrices**: 2x2 and 3x3 rotation matrices
- Full conversion support between all rotation representations

### Geometric Types
- **Point2D, Point3D**: Point types with vector offset operations
- **Pose2D, Pose3D**: Position and rotation combined (2D and 3D)
- **Plane**: 3D plane definition with normal and distance

## Building and Publishing

Build the WIT package:
```sh
wkg wit build
```

Push to OCI registry:
```sh
wkg oci push ardo314/math:x.x.x <file> -u <user> -p <token>
```

## CI/CD

A Dockerfile is provided for CI environments with the necessary toolchain:
- Rust (slim)
- cargo-component
- wasm-tools
- wkg

## Files

- `wit/world.wit` - WIT interface definitions
- `config.toml` - Registry configuration
- `ci.Dockerfile` - CI environment setup
