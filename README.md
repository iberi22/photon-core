# Photon-Core: 5D Optical Data Encoding Simulation Framework

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![arXiv](https://img.shields.io/badge/arXiv-cs.ET-b31b1b.svg)](https://arxiv.org/list/cs.ET/recent)

A high-performance Rust framework for simulating **5-dimensional optical data storage**. This software models how digital data can be encoded into photonic voxels using multiple physical properties of light: intensity, polarization, phase, and wavelength.

> **Research Context**: This framework provides an open-source testbed for exploring encoding algorithms suitable for next-generation optical storage systems such as [Microsoft Project Silica](https://www.microsoft.com/en-us/research/project/project-silica/) and 5D glass memory technologies.

---

## Table of Contents

- [Quick Start](#quick-start)
- [Theoretical Background](#theoretical-background)
- [Mathematical Model](#mathematical-model)
- [System Architecture](#system-architecture)
- [Encoding Scheme](#encoding-scheme)
- [Error Correction](#error-correction)
- [Steganographic Properties](#steganographic-properties)
- [Experimental Results](#experimental-results)
- [Reproduction Guide](#reproduction-guide)
- [API Reference](#api-reference)
- [Citation](#citation)
- [License](#license)

---

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) 1.75 or higher
- Cargo (included with Rust)

### Installation

```bash
git clone https://github.com/iberi22/photon-core.git
cd photon-core
cargo build --release
```

### Run Demo

```bash
cargo run --example demo
```

Expected output:
```
=== 5D Optical Data Storage PoC ===
Original Data: "Hello, 5D World!"
[Encoding] Generated 16 voxels.
[Decoding] Authorized Read (with noise):
Result: "Hello, 5D World!"
>> SUCCESS: Data recovered correctly despite noise.
[Security] Attempting Unauthorized Read (Ignoring Polarization)...
>> SECURITY VERIFIED: Unauthorized read failed to retrieve data.
```

---

## Theoretical Background

### 5D Optical Storage Concept

Traditional storage media (HDD, SSD, optical discs) encode data in **2D surfaces** or **3D volumes**. 5D optical storage extends this by exploiting additional **physical dimensions of light**:

| Dimension | Physical Property | Information Carrier |
|-----------|------------------|---------------------|
| **1-3** | Spatial Position (X, Y, Z) | Voxel location in crystal lattice |
| **4** | Intensity / Retardance | Amplitude of birefringent nanostructure |
| **5** | Polarization Orientation | Slow-axis angle of nanostructure |
| **+** | Phase | Optical path difference |
| **+** | Wavelength | Spectral multiplexing |

### Physical Basis

When a femtosecond laser pulse is focused inside fused silica glass, it creates **self-assembled nanogratings** through a nonlinear optical process. These nanogratings exhibit:

1. **Form Birefringence**: The nanostructure acts as a uniaxial crystal with controllable:
   - Slow-axis orientation (polarization angle)
   - Retardance magnitude (intensity)

2. **Spatial Selectivity**: Laser focus can be steered in 3D, creating volumetric data storage.

3. **Permanence**: Modifications are stable for billions of years at room temperature.

---

## Mathematical Model

### The Photonic Voxel

We define a **Photonic Voxel** as the atomic unit of storage:

$$\mathbf{V} = (I, \theta, \phi, \lambda)$$

Where:
- $I \in [0, 1]$: Normalized intensity (retardance magnitude)
- $\theta \in [0, \pi)$: Polarization angle (slow-axis orientation)
- $\phi \in [0, 2\pi)$: Optical phase
- $\lambda \in \mathbb{R}^+$: Wavelength in nanometers

### Encoding Function

The encoding function $E: \{0,1\}^8 \rightarrow \mathbf{V}$ maps an 8-bit byte to a voxel:

$$E(b) = \Big(I(b_{0:1}), \theta(b_{2:3}), \phi(b_{4:5}), \lambda(b_{6:7})\Big)$$

Where $b = b_7 b_6 b_5 b_4 b_3 b_2 b_1 b_0$ and:

**Intensity Mapping** (bits 0-1):

$$I(b_{0:1}) = (b_{0:1} + 1) \times 0.25$$

| $b_{0:1}$ | $I$ |
|-----------|-----|
| 00 | 0.25 |
| 01 | 0.50 |
| 10 | 0.75 |
| 11 | 1.00 |

**Polarization Mapping** (bits 2-3):

$$\theta(b_{2:3}) = b_{2:3} \times \frac{\pi}{4}$$

| $b_{2:3}$ | $\theta$ (rad) | $\theta$ (deg) |
|-----------|----------------|----------------|
| 00 | 0 | 0° |
| 01 | π/4 | 45° |
| 10 | π/2 | 90° |
| 11 | 3π/4 | 135° |

**Phase Mapping** (bits 4-5):

$$\phi(b_{4:5}) = b_{4:5} \times \frac{\pi}{2}$$

| $b_{4:5}$ | $\phi$ (rad) | $\phi$ (deg) |
|-----------|--------------|--------------|
| 00 | 0 | 0° |
| 01 | π/2 | 90° |
| 10 | π | 180° |
| 11 | 3π/2 | 270° |

**Wavelength Mapping** (bits 6-7):

$$\lambda(b_{6:7}) \in \{532, 650, 450, 800\} \text{ nm}$$

| $b_{6:7}$ | $\lambda$ (nm) | Color |
|-----------|----------------|-------|
| 00 | 532 | Green |
| 01 | 650 | Red |
| 10 | 450 | Blue |
| 11 | 800 | Near-IR |

### Decoding Function

The decoding function $D: \mathbf{V} \rightarrow \{0,1\}^8$ uses **nearest-neighbor quantization**:

$$D(\mathbf{V}) = \arg\min_b \|E(b) - \mathbf{V}\|$$

For each dimension, we find the closest discrete level:

$$b_{0:1} = \arg\min_i |I - (i+1) \times 0.25|, \quad i \in \{0,1,2,3\}$$

$$b_{2:3} = \arg\min_i |\theta - i \times \frac{\pi}{4}|, \quad i \in \{0,1,2,3\}$$

$$b_{4:5} = \arg\min_i |\phi - i \times \frac{\pi}{2}|, \quad i \in \{0,1,2,3\}$$

$$b_{6:7} = \arg\min_i |\lambda - \lambda_i|, \quad \lambda_i \in \{532, 650, 450, 800\}$$

### Noise Model

Physical readout introduces Gaussian noise to each dimension:

$$\mathbf{V}' = \mathbf{V} + \mathcal{N}(0, \sigma^2)$$

Where:
- $\sigma_I \approx 0.05$ (5% intensity noise)
- $\sigma_\theta \approx 0.08$ rad (≈4.6° polarization jitter)
- $\sigma_\phi \approx 0.10$ rad (≈5.7° phase noise)
- $\sigma_\lambda \approx 10$ nm (wavelength drift)

### Information Density

| Metric | Value |
|--------|-------|
| Bits per voxel | 8 (2 bits × 4 dimensions) |
| Bytes per voxel | 1 |
| Voxel memory footprint | 16 bytes (4 × f32) |
| Encoding overhead | 16:1 (simulation artifact) |

---

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      User Interface                          │
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  ┌─────────────────┐ │
│  │   CLI   │  │  Demo   │  │  Tests  │  │   Benchmarks    │ │
│  └────┬────┘  └────┬────┘  └────┬────┘  └────────┬────────┘ │
└───────┼────────────┼────────────┼────────────────┼──────────┘
        │            │            │                │
┌───────▼────────────▼────────────▼────────────────▼──────────┐
│                        photon_core                           │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                      codec.rs                         │   │
│  │  encode_data() ◄──────────────────► decode_data()    │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐            │
│  │ structs.rs │  │ physics.rs │  │ security.rs│            │
│  │ PhotonicV. │  │ Crosstalk  │  │ Stegano.   │            │
│  └────────────┘  └────────────┘  └────────────┘            │
│  ┌────────────┐  ┌────────────┐                             │
│  │   ecc.rs   │  │ analysis.rs│                             │
│  │ Reed-Sol.  │  │ BER Sim.   │                             │
│  └────────────┘  └────────────┘                             │
└─────────────────────────────────────────────────────────────┘
```

### Module Descriptions

| Module | Purpose |
|--------|---------|
| `structs.rs` | Defines `PhotonicVoxel` struct (16-byte aligned) |
| `codec.rs` | Bidirectional encoding/decoding with noise simulation |
| `ecc.rs` | Reed-Solomon error correction (10 data + 4 parity shards) |
| `physics.rs` | 3D crosstalk/ISI simulation |
| `security.rs` | Steganography demonstration |
| `analysis.rs` | Bit Error Rate (BER) simulation tools |

---

## Encoding Scheme

### Data Structure

```rust
#[repr(C)]
pub struct PhotonicVoxel {
    pub intensity: f32,    // [0.25, 1.0] - 2 bits
    pub polarization: f32, // [0, π) rad  - 2 bits
    pub phase: f32,        // [0, 2π) rad - 2 bits
    pub wavelength: f32,   // nm          - 2 bits
}
// Total: 16 bytes, cache-line friendly
```

### Bit Layout

```
Byte: [b7 b6 | b5 b4 | b3 b2 | b1 b0]
       ├──┬──┘ ├──┬──┘ ├──┬──┘ ├──┬──┘
       │  │    │  │    │  │    │  └── Intensity (2 bits)
       │  │    │  │    │  └─────────── Polarization (2 bits)
       │  │    │  └────────────────── Phase (2 bits)
       │  └───────────────────────── Wavelength (2 bits)
```

---

## Error Correction

### Reed-Solomon Configuration

| Parameter | Value |
|-----------|-------|
| Data shards | 10 |
| Parity shards | 4 |
| Total shards | 14 |
| Overhead | 40% |
| Correction capability | Up to 4 erasures or 2 errors |

### Implementation

```rust
// Add ECC to data
let protected = add_error_correction(&data);

// Recover with potential errors
let recovered = recover_error_correction(&protected)?;
```

---

## Steganographic Properties

### Security Through Dimensionality

The polarization dimension provides **inherent data hiding**:

1. **Authorized reader**: Knows to read all 4 dimensions → correct data
2. **Unauthorized reader**: Ignores polarization → corrupted data

### Demonstration

```rust
// Authorized read
let correct = decode_data(&voxels, false);  // ✓ Original data

// Simulated unauthorized read (polarization = 0)
let stolen = read_ignoring_polarization(&voxels);  // ✗ Garbage
```

### Bit Error Analysis

When polarization is ignored:
- Bits 2-3 (polarization) are always read as `00`
- Expected BER: **25%** (2 bits wrong out of 8)
- Actual data becomes **unrecoverable** without the polarization key

---

## Experimental Results

### Performance Benchmarks

| Operation | Time (1KB) | Throughput |
|-----------|------------|------------|
| Encoding | 2.1 µs | **476 MB/s** |
| Decoding (clean) | 9.6 µs | 104 MB/s |
| Decoding (noisy) | 22.5 µs | 44 MB/s |

### Bit Error Rate vs. Noise

Run BER experiment:
```bash
cargo run --release -- experiment --max-noise 0.3 --output ber_results.csv
```

Results:

| Noise Amplitude | BER |
|-----------------|-----|
| 0.00 | 0.00000 |
| 0.05 | 0.00000 |
| 0.10 | 0.00012 |
| 0.15 | 0.00891 |
| 0.20 | 0.03254 |
| 0.25 | 0.05513 |
| 0.30 | 0.07166 |

**Observation**: The codec tolerates up to ~6% noise amplitude before significant degradation, thanks to the discrete quantization levels providing noise margins.

---

## Reproduction Guide

### Step 1: Clone and Build

```bash
git clone https://github.com/iberi22/photon-core.git
cd photon-core
cargo build --release
```

### Step 2: Run Tests

```bash
cargo test
```

Expected:
```
running 5 tests
test test_empty_input ... ok
test test_round_trip_noiseless ... ok
test test_round_trip_with_noise ... ok
test test_steganography_effectiveness ... ok
test test_codec_roundtrip_noiseless ... ok

test result: ok. 5 passed
```

### Step 3: Run Demo

```bash
cargo run --example demo
```

### Step 4: CLI Usage

**Encode a file:**
```bash
echo "Hello, 5D optical storage!" > test.txt
cargo run --release -- encode --input test.txt --output test.vox --ecc
```

**Decode with noise simulation:**
```bash
cargo run --release -- decode --input test.vox --output recovered.txt --noise
cat recovered.txt
```

### Step 5: Run Benchmarks

```bash
cargo bench
```

### Step 6: Generate BER Data

```bash
cargo run --release -- experiment --max-noise 0.4 --output ber_data.csv
```

---

## API Reference

### Core Functions

```rust
use photon_core::{encode_data, decode_data, PhotonicVoxel};

// Encode bytes to voxels
let voxels: Vec<PhotonicVoxel> = encode_data(&data);

// Decode voxels to bytes (with optional noise simulation)
let recovered: Vec<u8> = decode_data(&voxels, simulate_noise);
```

### Error Correction

```rust
use photon_core::{add_error_correction, recover_error_correction};

// Add Reed-Solomon parity
let protected = add_error_correction(&data);

// Recover and verify
let original = recover_error_correction(&protected)?;
```

### Analysis

```rust
use photon_core::run_ber_simulation;

// Run BER experiment
let results = run_ber_simulation(
    data_size,   // bytes to test
    steps,       // noise level steps
    max_noise    // maximum noise amplitude
);
```

---

## Citation

If you use this software in your research, please cite:

```bibtex
@software{belalcazar2026photoncore,
  author       = {Belalcazar, Ivan},
  title        = {Photon-Core: A Rust-based Simulation Framework for 
                  High-Density 5D Optical Data Encoding},
  year         = {2026},
  publisher    = {GitHub},
  url          = {https://github.com/iberi22/photon-core}
}
```

Or use the `CITATION.cff` file in this repository.

---

## Related Work

- Wang, Y. et al. (2024). "5D optical data storage in silica glass." *Nature Photonics*.
- Zhang, Y. et al. (2025). "Multi-layer 5D Optical Data Storage: Mathematical Modeling and Deep Learning-Based Reconstruction." *arXiv:2508.20106*.
- Microsoft Research. "Project Silica." https://www.microsoft.com/en-us/research/project/project-silica/

---

## License

MIT License - see [LICENSE](LICENSE) file.

---

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

### Development Setup

```bash
# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

---

**Author**: Ivan Belalcazar ([@iberi22](https://github.com/iberi22))  
**Contact**: iberi22@gmail.com
