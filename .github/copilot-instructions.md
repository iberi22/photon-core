# GitHub Copilot Instructions for 5D Optical Encoding Research Framework

## 1. Project Overview & Architecture
This Rust workspace simulates **5D optical data storage**, modeling how digital data is encoded into nanostructured glass using femtosecond lasers.
- **Core Unit**: `PhotonicVoxel` (`src/structs.rs`) is the atomic unit of storage. It is strictly 16-byte aligned to support SIMD operations and represents a single point in the 3D crystal lattice.
- **Encoding Density**: 1 Byte per Voxel (8 bits).
  - **Intensity**: 2 bits (4 discrete levels).
  - **Polarization**: 2 bits (4 angles: 0°, 45°, 90°, 135°).
  - **Phase**: 2 bits (4 states: 0, π/2, π, 3π/2).
  - **Wavelength**: 2 bits (4 discrete colors: Red, Green, Blue, IR).
- **Service Boundaries**:
  - `codec.rs`: Pure logic for mapping `u8` <-> `PhotonicVoxel`.
  - `physics.rs`: Simulation of physical artifacts (Inter-Symbol Interference/Crosstalk).
  - `ecc.rs`: Error Correction (Reed-Solomon) layer on top of raw bytes.
  - `analysis.rs`: Statistical tools (Bit Error Rate).

## 2. Core Abstractions & Patterns

### The `PhotonicVoxel`
Always respect the 16-byte alignment preference.
```rust
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhotonicVoxel {
    pub intensity: f32,    // 0.25 to 1.0
    pub polarization: f32, // Radians [0, PI)
    pub phase: f32,        // Radians [0, 2*PI)
    pub wavelength: f32,   // Nanometers (e.g., 532.0)
}
```

### Encoding Map
When modifying `codec.rs`, adhere to this bitmask mapping:
- **Bits 0-1**: Intensity
- **Bits 2-3**: Polarization
- **Bits 4-5**: Phase
- **Bits 6-7**: Wavelength

### Dependencies/External Crates
- **`reed-solomon-erasure`**: Used for ECC. The implementation splits the *entire* data buffer into shards (10 data + 4 parity), rather than small internal blocks.
- **`rand`**: Used for Gaussian noise generation in read-simulations.
- **`criterion`**: Used for performance benchmarking (`benches/`).
- **`proptest`**: Used for property-based testing in `tests/`.

## 3. Developer Workflows & Commands

### Build & Run
- **Production Build**: Always use `--release` for simulation heavy-lifting to enable compiler optimizations.
  ```bash
  cargo build --release
  ```
- **CLI Usage**:
  ```bash
  # Encode
  cargo run --release -- encode --input <file> --output <file.vox> --ecc
  # Decode with Noise
  cargo run --release -- decode --input <file.vox> --output <file> --noise
  ```

### Testing Strategy
- **Unit Tests**: Place module-specific logic tests in `src/`.
- **Property Tests**: Use `proptest!` macros in `tests/` to verify round-trip integrity (`decode(encode(data)) == data`) with random byte arrays.
- **Benchmarks**: Critical for the `codec` loop.
  ```bash
  cargo bench
  ```

## 4. Physics & Simulation Guidelines
- **Noise Model**: When implementing noise, apply it to the floating-point values of the voxel *before* decoding steps back to discrete bits.
- **Crosstalk**: Modeled in `physics.rs`. When expanding this, assume a 3D grid context where `z` is derived from `index / (width * height)`.
- **Constants**: Use `std::f32::consts::PI` for all angular math. Wavelengths are defined in `codec.rs` as explicit `f32` const arrays.

## 5. Common Implementation Pitfalls
- **Wavelength Matching**: Wavelengths are floating point. Use epsilon comparisons or nearest-neighbor logic when deciding which discrete bit value simulates a read wavelength.
- **Data Padding**: The Reed-Solomon implementation requires input data to be padded to a multiple of the shard count (10). Ensure `ecc.rs` handles padding/stripping correctly.
