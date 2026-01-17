use clap::{Parser, Subcommand};
use std::fs;
use std::path::PathBuf;
use std::io::Write;
use photon_core::{encode_data, decode_data, add_error_correction, recover_error_correction, run_ber_simulation, PhotonicVoxel};

#[derive(Parser)]
#[command(name = "photon_cli")]
#[command(about = "5D Optical Storage Research CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Encodes a file into Photonic Voxels (simulated binary output)
    Encode {
        /// Input file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path (defaults to input.vox)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Add Error Correction
        #[arg(long)]
        ecc: bool,
    },
    /// Decodes a voxel file back to original data
    Decode {
        /// Input voxel file path
        #[arg(short, long)]
        input: PathBuf,

        /// Output file path
        #[arg(short, long)]
        output: PathBuf,

        /// Simulate readout noise
        #[arg(long)]
        noise: bool,
    },
    /// Runs a research experiment (BER Simulation)
    Experiment {
        /// Output CSV file path
        #[arg(short, long, default_value = "ber_results.csv")]
        output: PathBuf,

        /// Maximum noise level to test
        #[arg(long, default_value_t = 0.2)]
        max_noise: f32,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode { input, output, ecc } => {
            println!("Reading input file: {:?}", input);
            let data = fs::read(input).expect("Failed to read input file");

            if data.is_empty() {
                println!("Warning: Input file is empty.");
            }

            let data_to_encode = if *ecc {
                println!("Adding Error Correction (Reed-Solomon)...");
                add_error_correction(&data)
            } else {
                data
            };

            println!("Encoding {} bytes (Density: 8 bits/voxel)...", data_to_encode.len());
            let voxels = encode_data(&data_to_encode);
            println!("Generated {} voxels.", voxels.len());

            let voxel_bytes = unsafe {
                std::slice::from_raw_parts(
                    voxels.as_ptr() as *const u8,
                    voxels.len() * std::mem::size_of::<PhotonicVoxel>(),
                )
            };

            let output_path = output.clone().unwrap_or_else(|| {
                let mut p = input.clone();
                p.set_extension("vox");
                p
            });

            fs::write(&output_path, voxel_bytes).expect("Failed to write output file");
            println!("Saved to {:?}", output_path);
        }
        Commands::Decode { input, output, noise } => {
            println!("Reading voxel file: {:?}", input);
            let raw_bytes = fs::read(input).expect("Failed to read voxel file");

            let struct_size = std::mem::size_of::<PhotonicVoxel>();
            if raw_bytes.len() % struct_size != 0 {
                panic!("File size is not a multiple of Voxel size ({} bytes). Corrupt file?", struct_size);
            }

            let count = raw_bytes.len() / struct_size;
            let mut voxels = Vec::with_capacity(count);

            unsafe {
                let ptr = raw_bytes.as_ptr() as *const PhotonicVoxel;
                for i in 0..count {
                    // Safety: We use read_unaligned because Vec<u8> might not be aligned to f32 (4 bytes).
                    voxels.push(std::ptr::read_unaligned(ptr.add(i)));
                }
            }

            println!("Decoding {} voxels...", voxels.len());
            let decoded_raw = decode_data(&voxels, *noise);

            let final_data = if decoded_raw.len().is_multiple_of(14) {
                 println!("Auto-detect: Checking for ECC structure (14-byte blocks)...");
                 match recover_error_correction(&decoded_raw) {
                     Ok(corrected) => {
                         println!("ECC Verification: SUCCESS. Parity stripped.");
                         corrected
                     },
                     Err(_) => {
                         println!("ECC Verification: Failed or not ECC data. Saving raw output.");
                         decoded_raw
                     }
                 }
            } else {
                decoded_raw
            };

            fs::write(output, final_data).expect("Failed to write output file");
            println!("Decoded data saved to {:?}", output);
        }
        Commands::Experiment { output, max_noise } => {
            println!("Running BER Experiment...");
            println!("Max Noise: {}, Data Size: 10KB, Steps: 20", max_noise);

            let results = run_ber_simulation(10_000, 20, *max_noise);

            let mut file = fs::File::create(output).expect("Failed to create results file");
            writeln!(file, "NoiseLevel,BER,ErrorBits,TotalBits").unwrap();

            for res in &results {
                writeln!(file, "{:.4},{:.6},{},{}", res.noise_level, res.ber, res.error_bits, res.total_bits).unwrap();
            }

            println!("Simulation complete. Results saved to {:?}", output);

            // Print a small summary to stdout
            println!("\nSummary:");
            println!("Noise | BER");
            println!("------+-------");
            for res in results.iter().take(5) {
                println!("{:.3} | {:.5}", res.noise_level, res.ber);
            }
            println!("...   | ...");
            for res in results.iter().rev().take(3).rev() {
                 println!("{:.3} | {:.5}", res.noise_level, res.ber);
            }
        }
    }
}
