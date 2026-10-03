//! Dacantus - Made by Kacefier - Version 1.0 - WAV I/O
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

use crate::protocol::SAMPLE_RATE;
use anyhow::Result;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::path::Path;

pub fn write_wav(path: &Path, samples: &[f32]) -> Result<()> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec)?;
    for &s in samples {
        let v = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        writer.write_sample(v)?;
    }
    writer.finalize()?;
    Ok(())
}

pub fn read_wav(path: &Path) -> Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(path)?;
    let spec = reader.spec();
    if spec.sample_format != SampleFormat::Int || spec.bits_per_sample != 16 {
        anyhow::bail!(
            "unsupported WAV format: {} bit {:?} (need 16-bit int)",
                      spec.bits_per_sample,
                      spec.sample_format
        );
    }

    let ch = spec.channels as usize;
    let mut out = Vec::new();

    if ch == 1 {
        for s in reader.samples::<i16>() {
            out.push(s? as f32 / i16::MAX as f32);
        }
    } else {
        // Average channels to mono.
        let raw: Vec<i16> = reader
        .samples::<i16>()
        .collect::<std::result::Result<_, _>>()?;
        for frame in raw.chunks_exact(ch) {
            let sum: i32 = frame.iter().map(|&x| x as i32).sum();
            out.push((sum / ch as i32) as f32 / i16::MAX as f32);
        }
    }
    Ok(out)
}
