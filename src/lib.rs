pub mod structs;
pub mod codec;
pub mod security;
pub mod ecc;
pub mod analysis;
pub mod physics; // Export physics

// Re-export for easier access
pub use structs::PhotonicVoxel;
pub use codec::{encode_data, decode_data};
pub use security::{read_ignoring_polarization, verify_obfuscation};
pub use ecc::{add_error_correction, recover_error_correction};
pub use analysis::{run_ber_simulation, SimulationResult};
pub use physics::simulate_crosstalk;
