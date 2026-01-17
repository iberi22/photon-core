use crate::structs::PhotonicVoxel;
use crate::codec::{encode_data, decode_data};
use rand::Rng;

/// Result of a Bit Error Rate (BER) simulation run.
#[derive(Debug)]
pub struct SimulationResult {
    pub noise_level: f32,
    pub total_bits: usize,
    pub error_bits: usize,
    pub ber: f64,
}

/// Runs a BER simulation by varying noise levels.
///
/// `data_size`: Number of bytes to test per step.
/// `steps`: Number of noise steps (0.0 to max_noise).
/// `max_noise`: Maximum noise amplitude (e.g., 0.2).
pub fn run_ber_simulation(data_size: usize, steps: usize, max_noise: f32) -> Vec<SimulationResult> {
    let mut results = Vec::new();

    // Generate random test data
    let mut rng = rand::rng();
    let data: Vec<u8> = (0..data_size).map(|_| rng.random()).collect();
    let voxels = encode_data(&data); // Encode once (noiseless ideal crystal)

    for i in 0..=steps {
        let noise_level = (max_noise * i as f32) / steps as f32;

        // Decode with specific noise level
        // We need to modify `decode_data` or expose the noise parameter more flexibly.
        // Currently `decode_data` uses hardcoded noise ranges if `simulate_noise` is true.
        // We need a way to inject specific noise amplitude.
        // For now, we will assume `decode_data` is refactored or we simulate noise externally here.

        let noisy_voxels = apply_noise(&voxels, noise_level);
        let decoded = decode_data(&noisy_voxels, false); // Decode without *adding* more noise inside

        let error_bits = count_bit_errors(&data, &decoded);
        let total_bits = data.len() * 8;

        results.push(SimulationResult {
            noise_level,
            total_bits,
            error_bits,
            ber: error_bits as f64 / total_bits as f64,
        });
    }

    results
}

/// Applies Gaussian-like noise to voxels with a specific amplitude.
fn apply_noise(voxels: &[PhotonicVoxel], amplitude: f32) -> Vec<PhotonicVoxel> {
    // Handle 0.0 amplitude to avoid empty range panic
    if amplitude <= 0.0 {
        return voxels.to_vec();
    }
    let mut rng = rand::rng();
    voxels.iter().map(|v| {
        let mut new_v = *v;
        // Apply noise to all dimensions scaled by amplitude
        new_v.intensity += rng.random_range(-amplitude..amplitude);
        new_v.polarization += rng.random_range(-amplitude..amplitude);
        new_v.phase += rng.random_range(-amplitude..amplitude);
        new_v.wavelength += rng.random_range(-amplitude*100.0..amplitude*100.0); // Wavelength is larger magnitude
        new_v
    }).collect()
}

/// Counts the number of differing bits between two byte arrays.
fn count_bit_errors(original: &[u8], decoded: &[u8]) -> usize {
    let len = std::cmp::min(original.len(), decoded.len());
    let mut errors = 0;

    for i in 0..len {
        let xor = original[i] ^ decoded[i];
        errors += xor.count_ones() as usize;
    }

    // If lengths differ, count missing/extra bits as errors (simplified)
    if original.len() != decoded.len() {
        errors += (original.len() as isize - decoded.len() as isize).unsigned_abs() * 8;
    }

    errors
}
