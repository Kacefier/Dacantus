//! Dacantus - Made by Kacefier - Version 1.0 - Error
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

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DacantusError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("WAV error: {0}")]
    Wav(#[from] hound::Error),

    #[error("Sync pattern not found in audio stream")]
    SyncNotFound,

    #[error("Invalid magic header: not a Dacantus audio file")]
    BadMagic,

    #[error("Unsupported protocol version: {0}")]
    BadVersion(u8),

    #[error("Checksum mismatch: expected {expected:#010x}, got {actual:#010x}")]
    ChecksumMismatch { expected: u32, actual: u32 },

    #[error("Truncated stream at {0}")]
    Truncated(&'static str),

    #[error("Metadata error: {0}")]
    Metadata(String),
}

pub type Result<T> = std::result::Result<T, DacantusError>;
