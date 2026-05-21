pub mod analysis;
pub mod codec;
pub mod ecc;
pub mod physics;
pub mod security;
pub mod structs; // Export physics

// Re-export for easier access
pub use analysis::{run_ber_simulation, SimulationResult};
pub use codec::{decode_data, encode_data};
pub use ecc::{add_error_correction, recover_error_correction};
pub use physics::simulate_crosstalk;
pub use security::{read_ignoring_polarization, verify_obfuscation};
pub use structs::PhotonicVoxel;
