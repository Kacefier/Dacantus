//! Dacantus - Made by Kacefier - Version 1.0 - Main
//!
//! Copyright (C) 2026 Kacefier
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

mod codec;
mod demodulator;
mod error;
mod modulator;
mod protocol;
mod wavio;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use anyhow::{bail, Context, Result};
use codec::{decode, encode};
use protocol::*;

/// Program version (CLI banner). Distinct from `PROTOCOL_VERSION`.
const VERSION: &str = "1.0";

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            print_help();
            Ok(())
        }
        "-v" | "--version" => {
            print_version();
            Ok(())
        }
        "-e" | "--encode" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for {}", args[1]);
                process::exit(1);
            }
            run_encode(&args[2])
        }
        "-d" | "--decode" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for {}", args[1]);
                process::exit(1);
            }
            run_decode(&args[2])
        }
        "-i" | "--info" => {
            if args.len() < 3 {
                eprintln!("Error: missing file argument for {}", args[1]);
                process::exit(1);
            }
            run_info(&args[2])
        }
        other => {
            eprintln!("Error: missing option for file: {}", other);
            eprintln!("Use one of: -e, -d, -i");
            eprintln!("Try 'dacantus --help' for usage.");
            process::exit(1);
        }
    }
}

fn print_version() {
    println!("Dacantus v{} - Made by Kacefier", VERSION);
    println!("GitHub: github.com/Kacefier/Dacantus");
    println!("Email: kacefier@zohomail.com");
    println!("This is free software, licensed under GPLv3+, See --help for details.");
}

fn print_help() {
    println!("Dacantus v{} - Made by Kacefier", VERSION);
    println!("GitHub: github.com/Kacefier/Dacantus");
    println!("Email: kacefier@zohomail.com");
    println!();
    println!("Usage: dacantus [OPTION] <FILE>");
    println!("Options:");
    println!("  -h, --help            Show this help message and exit");
    println!("  -v, --version         Show version information and exit");
    println!("  -e, --encode <file>   Encode a file into a .wav next to it");
    println!("  -d, --decode <file>   Decode a .wav back into the original file");
    println!("  -i, --info <file>     Show info about a Dacantus .wav, no output written");
    println!();
    println!("Examples:");
    println!("  dacantus -e a.txt          Produces a.wav in the same directory");
    println!("  dacantus -d a.wav          Produces a.txt in the same directory");
    println!("  dacantus -i a.wav          Print file name, size and duration");
    println!();
    println!("Protocol:");
    println!("  2-FSK @ 2400 baud, 9600 Hz mono, DCT1 container, CRC-32 per frame.");
    println!();
    println!("Copyright and license:");
    println!("  Copyright (C) 2026 Kacefier");
    println!("  Dacantus is free software licensed under the GNU General Public");
    println!("  License, version 3 or (at your option) any later version.");
    println!("  This program is distributed in the hope that it will be useful,");
    println!("  but WITHOUT ANY WARRANTY; without even the implied warranty of");
    println!("  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.");
    println!();
    println!("  See <https://www.gnu.org/licenses/> for the full license text.");
}

fn run_encode(input: &str) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        bail!("input file not found: {}", input_path.display());
    }

    let data = fs::read(input_path)
    .with_context(|| format!("reading {}", input_path.display()))?;

    let file_name = input_path
    .file_name()
    .and_then(|s| s.to_str())
    .context("invalid input file name")?;

    let samples = encode(file_name, &data)?;
    let output = with_extension(input_path, "wav");
    wavio::write_wav(&output, &samples)
    .with_context(|| format!("writing {}", output.display()))?;

    println!("encoded {} -> {}", input_path.display(), output.display());
    Ok(())
}

fn run_decode(input: &str) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        bail!("input file not found: {}", input_path.display());
    }

    let samples = wavio::read_wav(input_path)
    .with_context(|| format!("reading {}", input_path.display()))?;

    let decoded = decode(&samples)?;

    let out_dir = input_path.parent().unwrap_or_else(|| Path::new("."));
    let out_path = out_dir.join(&decoded.file_name);
    fs::write(&out_path, &decoded.data)
    .with_context(|| format!("writing {}", out_path.display()))?;

    println!("decoded {} -> {}", input_path.display(), out_path.display());
    Ok(())
}

fn run_info(input: &str) -> Result<()> {
    let input_path = Path::new(input);
    if !input_path.exists() {
        bail!("input file not found: {}", input_path.display());
    }

    let samples = wavio::read_wav(input_path)
    .with_context(|| format!("reading {}", input_path.display()))?;
    let decoded = decode(&samples)?;

    println!("File name : {}", decoded.file_name);
    println!("File size : {} bytes", decoded.data.len());
    println!("Audio     : {} samples @ {} Hz", samples.len(), SAMPLE_RATE);
    println!(
        "Duration  : {:.2} s",
        samples.len() as f32 / SAMPLE_RATE as f32
    );
    Ok(())
}

fn with_extension(p: &Path, ext: &str) -> PathBuf {
    let mut out = p.to_path_buf();
    out.set_extension(ext);
    out
}
