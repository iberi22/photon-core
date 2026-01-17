/// Represents a single unit of data storage in the 5D optical memory crystal.
///
/// This struct models the physical properties of a laser pulse used to write
/// or read a voxel (volumetric pixel) within the crystal lattice.
///
/// Dimensions modeled:
/// 1. Intensity (Amplitude)
/// 2. Polarization Angle
/// 3. Phase (Optical path difference)
/// 4. Wavelength (Color/Frequency)
///
/// Optimized for memory alignment (16 bytes) to support SIMD operations.
/// We use #[repr(C)] to ensure the layout corresponds to C struct layout,
/// which with 4 f32s will be tightly packed and aligned to 4 bytes, 
/// but the overall size is 16 bytes, fitting nicely into SIMD registers.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhotonicVoxel {
    /// Optical Intensity (Amplitude squared). Normalized range [0.0, 1.0].
    /// Used to encode 2 bits in the PoC.
    pub intensity: f32,

    /// Polarization Angle in radians. Range [0, PI).
    /// Used to encode 2 bits in the PoC (0°, 45°, 90°, 135°).
    pub polarization: f32,

    /// Optical Phase shift in radians. Range [0, 2*PI).
    /// Can be used for additional multiplexing or holographic reconstruction.
    pub phase: f32,

    /// Wavelength in nanometers (nm).
    /// Allows for spectral multiplexing (saving data at different colors).
    pub wavelength: f32,
}

impl PhotonicVoxel {
    /// Creates a new PhotonicVoxel with specified physical properties.
    pub fn new(intensity: f32, polarization: f32, phase: f32, wavelength: f32) -> Self {
        Self {
            intensity,
            polarization,
            phase,
            wavelength,
        }
    }
}
