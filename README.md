# 5D Optical Data Storage Simulation

This repository contains a Rust-based research framework for simulating 5D optical data storage. It models the physical properties of light to encode digital data into a volumetric crystal structure.

## Features

- **5D Encoding**: Utilizes Position (x,y,z), Intensity, Polarization, Phase, and Wavelength.
- **High Density**: Stores 8 bits (1 byte) per voxel using 4 properties.
- **Physics-based Voxel**: `PhotonicVoxel` struct aligned for performance (16 bytes).
- **Steganography**: Demonstrates how data is hidden in the polarization dimension.
- **Noise Simulation**: Simulates physical readout noise (Gaussian).
- **Error Correction**: Integrated Reed-Solomon coding (Proof of Concept).
- **CLI**: Command-line tool for encoding/decoding files.
- **Benchmarks**: `criterion` benchmarks for performance analysis.

## Usage

### Build
```bash
cargo build --release
```

### Run Demo
```bash
cargo run --example demo
```

### CLI
Encode a file (automatically adds ECC if requested):
```bash
cargo run --release -- encode --input secret.txt --output crystal.vox --ecc
```

Decode a file (with noise simulation):
```bash
cargo run --release -- decode --input crystal.vox --output recovered.txt --noise
```

## Physics Model

Each voxel stores 8 bits of information (1 Byte):
- **Intensity**: 2 bits (4 discrete levels: 0.25, 0.50, 0.75, 1.00).
- **Polarization**: 2 bits (0°, 45°, 90°, 135°).
- **Phase**: 2 bits (0, π/2, π, 3π/2).
- **Wavelength**: 2 bits (532nm, 650nm, 450nm, 800nm).

## Benchmarks

Run benchmarks using:
```bash
cargo bench
```

Approximate performance (1KB block):
- Encoding: ~4.5 µs
- Decoding (Noiseless): ~21 µs
- Decoding (Noisy): ~55 µs

## License

MIT
