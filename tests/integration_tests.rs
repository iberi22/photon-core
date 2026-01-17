use photon_core::{encode_data, decode_data, read_ignoring_polarization, verify_obfuscation};

#[test]
fn test_round_trip_noiseless() {
    let data = b"Hello World";
    let voxels = encode_data(data);
    let decoded = decode_data(&voxels, false);
    // Decoded might have trailing zeros due to 3-byte chunk alignment.
    // We check that the prefix matches.
    assert!(decoded.starts_with(data), "Noiseless round trip failed: Decoded {:?} vs Original {:?}", decoded, data);
}

#[test]
fn test_round_trip_with_noise() {
    // With noise, we might have bit flips if noise is too high or ECC is missing.
    // For now, the codec uses a simple nearest-neighbor approach which handles small noise.
    // The default noise in codec is 5%, which *should* usually be fine for our discrete levels (0.25 separation).
    let data = b"Testing Noise";
    let voxels = encode_data(data);
    let decoded = decode_data(&voxels, true);
    
    // Check prefix
    assert!(decoded.starts_with(data), "Noisy round trip failed: Decoded {:?} vs Original {:?}", decoded, data);
}

#[test]
fn test_steganography_effectiveness() {
    let data = b"Secret Data";
    let voxels = encode_data(data);
    
    // Unauthorized read
    let stolen = read_ignoring_polarization(&voxels);
    
    // Should NOT match
    assert_ne!(data.as_slice(), stolen.as_slice(), "Steganography failed: data leaked");
    
    // Verify using the helper
    assert!(verify_obfuscation(data, &voxels));
}

#[test]
fn test_empty_input() {
    let data = b"";
    let voxels = encode_data(data);
    assert!(voxels.is_empty());
    let decoded = decode_data(&voxels, false);
    assert!(decoded.is_empty());
}
