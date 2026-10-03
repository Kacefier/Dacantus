//! Dacantus - Made by Kacefier - Version 1.0 - Protocol
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
//! Dacantus wire protocol constants.
//!
//! Physical layer: 2-FSK at 2400 baud, 9600 Hz mono.
//! bit 0 -> MARK_FREQ, bit 1 -> SPACE_FREQ.

pub const SAMPLE_RATE: u32 = 9_600;
pub const BAUD: u32 = 2_400;
pub const SAMPLES_PER_BIT: u32 = SAMPLE_RATE / BAUD; // 4

pub const MARK_FREQ: f32 = 1_800.0;
pub const SPACE_FREQ: f32 = 3_600.0;

/// Preamble: 0xAA repeated -> bit pattern 10101010...
pub const PREAMBLE_BYTES: usize = 32;

/// Magic "DCT1" that follows the preamble.
pub const MAGIC: [u8; 4] = *b"DCT1";

/// Protocol version byte inside the DCT1 container.
pub const PROTOCOL_VERSION: u8 = 1;

pub const FRAME_MAGIC: [u8; 4] = *b"FRM1";
pub const END_MAGIC: [u8; 4] = *b"END1";

pub const FRAME_PAYLOAD_MAX: usize = 256;
pub const FILE_NAME_MAX: usize = 255;
