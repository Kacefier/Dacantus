//! Dacantus - Made by Kacefier - Version 1.0 - Modulator
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
//!
//! Bit-level 2-FSK modulation: bits -> f32 PCM samples.

use crate::protocol::*;

/// Convert a bit vector into PCM samples using phase-continuous 2-FSK.
/// Phase is reset per bit so that the tone energy stays well-defined.
pub fn modulate_bits(bits: &[u8]) -> Vec<f32> {
    let mut samples = Vec::with_capacity(bits.len() * SAMPLES_PER_BIT as usize);
    let two_pi = std::f32::consts::TAU;

    for &b in bits {
        let freq = if b == 0 { MARK_FREQ } else { SPACE_FREQ };
        let phase_inc = two_pi * freq / SAMPLE_RATE as f32;
        let mut phase = 0.0f32;
        for _ in 0..SAMPLES_PER_BIT {
            samples.push(phase.sin() * 0.8);
            phase += phase_inc;
            if phase > two_pi {
                phase -= two_pi;
            }
        }
    }

    apply_fade(&mut samples, SAMPLES_PER_BIT as usize);
    samples
}

fn apply_fade(samples: &mut [f32], n: usize) {
    let f = n.min(samples.len() / 2);
    for i in 0..f {
        let g = i as f32 / f as f32;
        samples[i] *= g;
        samples[samples.len() - 1 - i] *= g;
    }
}

pub fn push_byte(bits: &mut Vec<u8>, byte: u8) {
    for i in (0..8).rev() {
        bits.push((byte >> i) & 1);
    }
}

pub fn push_bytes(bits: &mut Vec<u8>, bytes: &[u8]) {
    for &b in bytes {
        push_byte(bits, b);
    }
}

pub fn push_u16_le(bits: &mut Vec<u8>, v: u16) {
    push_bytes(bits, &v.to_le_bytes());
}

pub fn push_u32_le(bits: &mut Vec<u8>, v: u32) {
    push_bytes(bits, &v.to_le_bytes());
}

pub fn push_u64_le(bits: &mut Vec<u8>, v: u64) {
    push_bytes(bits, &v.to_le_bytes());
}
