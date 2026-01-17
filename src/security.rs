use crate::structs::PhotonicVoxel;
use crate::codec::decode_data;

/// Demonstrates Steganography by simulating a reader that ignores Polarization.
///
/// If a standard optical reader (or an unauthorized one) only reads Intensity,
/// they will misinterpret the data.
///
/// This function simulates reading only the Intensity dimension and assuming
/// the Polarization bits are 0 (or random).
///
/// Returns the byte array as interpreted by this "ignorant" reader.
pub fn read_ignoring_polarization(voxels: &[PhotonicVoxel]) -> Vec<u8> {
    // We reuse the decode logic but we tamper with the voxels first.
    // We create a copy of voxels where polarization is set to 0.
    let mutated_voxels: Vec<PhotonicVoxel> = voxels.iter().map(|v| {
        let mut new_v = *v;
        new_v.polarization = 0.0; // Force polarization to 0 (Level 0)
        new_v
    }).collect();

    // Decode without noise (deterministic failure)
    decode_data(&mutated_voxels, false)
}

/// Verifies that the "ignorant" read does not match the original data.
/// Returns true if the data is successfully obfuscated (i.e., decrypted data != original).
pub fn verify_obfuscation(original: &[u8], voxels: &[PhotonicVoxel]) -> bool {
    let unauthorized_read = read_ignoring_polarization(voxels);
    
    // Check if the unauthorized read matches original.
    // It should NOT match.
    if original == unauthorized_read {
        return false;
    }
    
    // Calculate how different it is?
    // For a random input, about 75% of nibbles should be wrong (since 2 bits are lost).
    // But we just return boolean success here.
    true
}
