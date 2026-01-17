use crate::structs::PhotonicVoxel;

/// Simulates 3D Cross-talk (Inter-Symbol Interference) in a crystal lattice.
///
/// This function simulates the effect of neighboring voxels "leaking" energy into
/// the target voxel due to diffraction limits (point spread function).
///
/// `voxels`: The linear sequence of voxels.
/// `width`: The width of the 2D plane (x-axis).
/// `height`: The height of the 2D plane (y-axis).
/// The z-axis (depth) is inferred from the length.
/// `crosstalk_factor`: The fraction of energy leaked from neighbors (e.g., 0.01).
pub fn simulate_crosstalk(voxels: &[PhotonicVoxel], width: usize, height: usize, crosstalk_factor: f32) -> Vec<PhotonicVoxel> {
    if width == 0 || height == 0 {
        return voxels.to_vec();
    }

    let layer_size = width * height;
    let depth = voxels.len().div_ceil(layer_size);
    let mut output = voxels.to_vec();

    // Helper to get index
    let get_idx = |x: usize, y: usize, z: usize| -> Option<usize> {
        if x >= width || y >= height || z >= depth {
            None
        } else {
            let idx = z * layer_size + y * width + x;
            if idx < voxels.len() { Some(idx) } else { None }
        }
    };

    for z in 0..depth {
        for y in 0..height {
            for x in 0..width {
                if let Some(target_idx) = get_idx(x, y, z) {
                    let mut original = voxels[target_idx];

                    // Neighbors (6-connectivity for simplicity: left, right, up, down, front, back)
                    let neighbors = [
                        (x.wrapping_sub(1), y, z), (x + 1, y, z),
                        (x, y.wrapping_sub(1), z), (x, y + 1, z),
                        (x, y, z.wrapping_sub(1)), (x, y, z + 1)
                    ];

                    for &(nx, ny, nz) in &neighbors {
                        // Check bounds (wrapping_sub handles < 0 check via usize overflow, but we must check max)
                        // Actually usize wrap causes huge number, so we check < width/height/depth
                        if nx < width && ny < height && nz < depth {
                             if let Some(n_idx) = get_idx(nx, ny, nz) {
                                 let neighbor = voxels[n_idx];
                                 // Add a fraction of neighbor's intensity to this voxel
                                 // Simplified model: intensity adds up
                                 original.intensity += neighbor.intensity * crosstalk_factor;

                                 // Polarization might rotate slightly? For now just intensity leakage.
                             }
                        }
                    }

                    // Clamp intensity to 1.0 + some headroom? Or let it bloom?
                    // Physics: Detectors saturate. Let's clamp at 1.5 just to see effect but not blow up f32.
                    if original.intensity > 1.5 { original.intensity = 1.5; }

                    output[target_idx] = original;
                }
            }
        }
    }
    output
}
