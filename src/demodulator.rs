//! Dacantus - Made by Kacefier - Version 1.0 - Demodulator
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
//! FSK demodulation via quadrature correlation (Goertzel-style).

use crate::protocol::*;

/// Correlate the window against a complex exponential at `freq`,
/// return magnitude^2.
fn tone_energy(window: &[f32], freq: f32) -> f32 {
    let two_pi = std::f32::consts::TAU;
    let mut re = 0.0f32;
    let mut im = 0.0f32;
    for (n, &s) in window.iter().enumerate() {
        let ph = two_pi * freq * (n as f32) / SAMPLE_RATE as f32;
        re += s * ph.cos();
        im += s * ph.sin();
    }
    re * re + im * im
}

/// Demodulate one bit starting at `pos` (must have at least SAMPLES_PER_BIT samples).
#[inline]
fn bit_at(samples: &[f32], pos: usize) -> u8 {
    let end = pos + SAMPLES_PER_BIT as usize;
    let w = &samples[pos..end];
    let e_mark = tone_energy(w, MARK_FREQ);
    let e_space = tone_energy(w, SPACE_FREQ);
    if e_space > e_mark { 1 } else { 0 }
}

/// Demodulate `n` bits starting at `pos`. Returns None if not enough samples.
fn bits_from(samples: &[f32], pos: usize, n: usize) -> Option<Vec<u8>> {
    let needed = n * SAMPLES_PER_BIT as usize;
    if pos + needed > samples.len() {
        return None;
    }
    let mut out = Vec::with_capacity(n);
    let mut p = pos;
    for _ in 0..n {
        out.push(bit_at(samples, p));
        p += SAMPLES_PER_BIT as usize;
    }
    Some(out)
}

/// Pack MSB-first bits into bytes.
fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.chunks_exact(8) {
        let mut b = 0u8;
        for &bit in chunk {
            b = (b << 1) | (bit & 1);
        }
        out.push(b);
    }
    out
}

/// Find the sample offset where the DCT1 magic begins.
///
/// We try every sub-bit offset (there are `SAMPLES_PER_BIT` of them), skip
/// the preamble, demodulate the next 32 bits, and check whether they spell
/// `DCT1`. That makes the lock unambiguous: no magic -> no lock.
pub fn find_data_start(samples: &[f32]) -> Option<usize> {
    let bit_len = SAMPLES_PER_BIT as usize;
    let preamble_bits = PREAMBLE_BYTES * 8;
    let preamble_samples = preamble_bits * bit_len;

    // 32 bits = MAGIC (4 bytes).
    const PROBE_BITS: usize = 32;

    for offset in 0..bit_len {
        // Try assuming bit-boundary alignment at `offset`.
        let data_pos = offset + preamble_samples;
        let bits = match bits_from(samples, data_pos, PROBE_BITS) {
            Some(b) => b,
            None => continue,
        };
        let bytes = bits_to_bytes(&bits);
        if bytes.len() >= 4 && bytes[..4] == MAGIC {
            return Some(data_pos);
        }
    }
    None
}

/// Demodulate the rest of the stream starting at `start_sample`.
/// Returns the packed bytes (trailing partial byte dropped).
pub fn demodulate_all(samples: &[f32], start_sample: usize) -> Vec<u8> {
    let bit_len = SAMPLES_PER_BIT as usize;
    let total_bits = (samples.len() - start_sample) / bit_len;
    let bits = match bits_from(samples, start_sample, total_bits) {
        Some(b) => b,
        None => return Vec::new(),
    };
    bits_to_bytes(&bits)
}
