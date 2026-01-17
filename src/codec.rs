use crate::structs::PhotonicVoxel;
use std::f32::consts::PI;
use rand::Rng;

// Constants for encoding
const INTENSITY_LEVELS: usize = 4;
const POLARIZATION_LEVELS: usize = 4;
const PHASE_LEVELS: usize = 4;

// Available Wavelengths (colors) in nanometers
// 0: Green (532 nm)
// 1: Red (650 nm)
// 2: Blue (450 nm)
// 3: IR (800 nm) - Just an example
const WAVELENGTHS: [f32; 4] = [532.0, 650.0, 450.0, 800.0];

/// Encodes a byte array into a vector of PhotonicVoxels using 8-bit encoding per voxel.
///
/// We are encoding 4 chunks of 2 bits each into one voxel:
/// - 2 bits -> Intensity (4 levels)
/// - 2 bits -> Polarization (4 angles)
/// - 2 bits -> Phase (4 angles)
/// - 2 bits -> Wavelength (4 colors)
///
/// Total: 8 bits (1 byte) per voxel.
///
/// This simplifies the encoding logic significantly: 1 byte -> 1 voxel.
pub fn encode_data(data: &[u8]) -> Vec<PhotonicVoxel> {
    let mut voxels = Vec::with_capacity(data.len());

    for &byte in data {
        voxels.push(encode_byte_to_voxel(byte));
    }

    voxels
}

/// Encodes a full byte into a single PhotonicVoxel.
/// Bits 0-1: Intensity
/// Bits 2-3: Polarization
/// Bits 4-5: Phase
/// Bits 6-7: Wavelength
fn encode_byte_to_voxel(byte: u8) -> PhotonicVoxel {
    let intensity_bits = byte & 0b0011;
    let polarization_bits = (byte >> 2) & 0b0011;
    let phase_bits = (byte >> 4) & 0b0011;
    let wavelength_bits = (byte >> 6) & 0b0011;

    // Intensity: [0.25, 0.5, 0.75, 1.0]
    let intensity = (intensity_bits as f32 + 1.0) * 0.25;

    // Polarization: [0, 45, 90, 135] (deg) -> [0, PI/4, PI/2, 3PI/4]
    let polarization = (polarization_bits as f32) * (PI / 4.0);

    // Phase: [0, 90, 180, 270] (deg) -> [0, PI/2, PI, 3PI/2]
    let phase = (phase_bits as f32) * (PI / 2.0);

    // Wavelength
    let wavelength = WAVELENGTHS[wavelength_bits as usize];

    PhotonicVoxel::new(intensity, polarization, phase, wavelength)
}

/// Decodes a vector of PhotonicVoxels back into bytes.
///
/// Simulates readout noise if `simulate_noise` is true.
pub fn decode_data(voxels: &[PhotonicVoxel], simulate_noise: bool) -> Vec<u8> {
    let mut data = Vec::with_capacity(voxels.len());

    for &voxel in voxels {
        data.push(decode_voxel(voxel, simulate_noise));
    }

    data
}

/// Decodes a single voxel into a byte.
fn decode_voxel(voxel: PhotonicVoxel, noise: bool) -> u8 {
    let mut intensity = voxel.intensity;
    let mut polarization = voxel.polarization;
    let mut phase = voxel.phase;
    let mut wavelength = voxel.wavelength;

    if noise {
        let mut rng = rand::rng();
        // Add Gaussian-like noise
        let i_noise: f32 = rng.random_range(-0.05..0.05);
        let p_noise: f32 = rng.random_range(-0.08..0.08);
        let ph_noise: f32 = rng.random_range(-0.1..0.1);
        let w_noise: f32 = rng.random_range(-10.0..10.0); // +/- 10nm noise

        intensity += i_noise;
        polarization += p_noise;
        phase += ph_noise;
        wavelength += w_noise;
    }

    // Decode Intensity
    let mut best_i_idx = 0;
    let mut best_i_dist = f32::MAX;
    for i in 0..INTENSITY_LEVELS {
        let level = (i as f32 + 1.0) * 0.25;
        let dist = (intensity - level).abs();
        if dist < best_i_dist {
            best_i_dist = dist;
            best_i_idx = i;
        }
    }

    // Decode Polarization
    let mut best_p_idx = 0;
    let mut best_p_dist = f32::MAX;
    for i in 0..POLARIZATION_LEVELS {
        let angle = (i as f32) * (PI / 4.0);
        let mut dist = (polarization - angle).abs();
        if dist > PI / 2.0 {
             dist = PI - dist;
        }

        if dist < best_p_dist {
            best_p_dist = dist;
            best_p_idx = i;
        }
    }

    // Decode Phase
    let mut best_ph_idx = 0;
    let mut best_ph_dist = f32::MAX;
    for i in 0..PHASE_LEVELS {
        let angle = (i as f32) * (PI / 2.0);
        let mut dist = (phase - angle).abs();
        if dist > PI {
            dist = (2.0 * PI) - dist;
        }

        if dist < best_ph_dist {
            best_ph_dist = dist;
            best_ph_idx = i;
        }
    }

    // Decode Wavelength
    let mut best_w_idx = 0;
    let mut best_w_dist = f32::MAX;
    for (i, &target) in WAVELENGTHS.iter().enumerate() {
        let dist = (wavelength - target).abs();
        if dist < best_w_dist {
            best_w_dist = dist;
            best_w_idx = i;
        }
    }

    let i_bits = best_i_idx as u8;
    let p_bits = best_p_idx as u8;
    let ph_bits = best_ph_idx as u8;
    let w_bits = best_w_idx as u8;

    // Reassemble: w_bits (6,7) | ph_bits (4,5) | p_bits (2,3) | i_bits (0,1)
    (w_bits << 6) | (ph_bits << 4) | (p_bits << 2) | i_bits
}
