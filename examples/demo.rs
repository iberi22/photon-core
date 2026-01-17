use photon_core::{encode_data, decode_data, verify_obfuscation, read_ignoring_polarization};

fn main() {
    println!("=== 5D Optical Data Storage PoC ===");

    // 1. Prepare Data
    let original_message = "Hello, 5D World!".as_bytes();
    println!("Original Data: {:?}", String::from_utf8_lossy(original_message));
    println!("Hex: {:02X?}", original_message);

    // 2. Encode
    let voxels = encode_data(original_message);
    println!("\n[Encoding] Generated {} voxels.", voxels.len());
    if let Some(first_voxel) = voxels.first() {
        println!("Sample Voxel 0: {:?}", first_voxel);
    }

    // 3. Decode (Authorized)
    let decoded_bytes = decode_data(&voxels, true); // With noise simulation
    println!("\n[Decoding] Authorized Read (with noise):");
    println!("Result: {:?}", String::from_utf8_lossy(&decoded_bytes));
    println!("Hex: {:02X?}", decoded_bytes);

    if original_message == decoded_bytes.as_slice() {
        println!(">> SUCCESS: Data recovered correctly despite noise.");
    } else {
        println!(">> WARNING: Data corruption occurred (check noise levels).");
    }

    // 4. Decode (Unauthorized / Steganography Check)
    println!("\n[Security] Attempting Unauthorized Read (Ignoring Polarization)...");
    let stolen_bytes = read_ignoring_polarization(&voxels);
    println!("Stolen Data: {:?}", String::from_utf8_lossy(&stolen_bytes));
    println!("Hex: {:02X?}", stolen_bytes);

    if verify_obfuscation(original_message, &voxels) {
        println!(">> SECURITY VERIFIED: Unauthorized read failed to retrieve data.");
    } else {
        println!(">> SECURITY FAILURE: Data leaked!");
    }
}
