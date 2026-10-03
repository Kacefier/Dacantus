//! Dacantus - Made by Kacefier - Version 1.0 - Codec
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
//! High-level file <-> audio codec.

use crate::demodulator::{demodulate_all, find_data_start};
use crate::error::{DacantusError, Result};
use crate::modulator::*;
use crate::protocol::*;
use crc32fast::Hasher;

pub struct EncodedFile {
    pub file_name: String,
    pub data: Vec<u8>,
}

fn crc32(data: &[u8]) -> u32 {
    let mut h = Hasher::new();
    h.update(data);
    h.finalize()
}

/// Encode file bytes + name into PCM samples.
pub fn encode(file_name: &str, data: &[u8]) -> Result<Vec<f32>> {
    let name_bytes = file_name.as_bytes();
    if name_bytes.len() > FILE_NAME_MAX {
        return Err(DacantusError::Metadata(format!(
            "file name too long ({} > {})",
                                                   name_bytes.len(),
                                                   FILE_NAME_MAX
        )));
    }

    let mut bits: Vec<u8> = Vec::new();

    // Preamble: 0xAA repeated.
    for _ in 0..PREAMBLE_BYTES {
        push_byte(&mut bits, 0xAA);
    }

    // Header: MAGIC | VERSION | name_len | name | total_len | crc32(payload)
    push_bytes(&mut bits, &MAGIC);
    push_byte(&mut bits, PROTOCOL_VERSION);
    push_byte(&mut bits, name_bytes.len() as u8);
    push_bytes(&mut bits, name_bytes);
    push_u64_le(&mut bits, data.len() as u64);
    push_u32_le(&mut bits, crc32(data));

    // Frames: FRAME_MAGIC | index | len | payload | crc32(payload)
    for (idx, chunk) in data.chunks(FRAME_PAYLOAD_MAX).enumerate() {
        push_bytes(&mut bits, &FRAME_MAGIC);
        push_u32_le(&mut bits, idx as u32);
        push_u16_le(&mut bits, chunk.len() as u16);
        push_bytes(&mut bits, chunk);
        push_u32_le(&mut bits, crc32(chunk));
    }

    push_bytes(&mut bits, &END_MAGIC);

    Ok(modulate_bits(&bits))
}

/// Decode PCM samples back into file name + bytes.
pub fn decode(samples: &[f32]) -> Result<EncodedFile> {
    let start = find_data_start(samples).ok_or(DacantusError::SyncNotFound)?;
    let bytes = demodulate_all(samples, start);

    let mut cur = ByteCursor::new(&bytes);

    // Header.
    let magic = cur.take(4).ok_or(DacantusError::Truncated("magic"))?;
    if magic != MAGIC {
        return Err(DacantusError::BadMagic);
    }
    let version = cur.u8().ok_or(DacantusError::Truncated("version"))?;
    if version != PROTOCOL_VERSION {
        return Err(DacantusError::BadVersion(version));
    }
    let name_len = cur.u8().ok_or(DacantusError::Truncated("name_len"))? as usize;
    let name_bytes = cur.take(name_len).ok_or(DacantusError::Truncated("file_name"))?;
    let file_name = String::from_utf8(name_bytes.to_vec())
    .map_err(|e| DacantusError::Metadata(format!("file name not UTF-8: {}", e)))?;
    let total_len = cur.u64().ok_or(DacantusError::Truncated("total_len"))? as usize;
    let expected_crc = cur.u32().ok_or(DacantusError::Truncated("payload_crc"))?;

    // Frames.
    let mut out = vec![0u8; total_len];
    let mut written = 0usize;

    loop {
        let peek = cur.peek(4).ok_or(DacantusError::Truncated("frame/end magic"))?;
        if peek == END_MAGIC {
            cur.take(4);
            break;
        }
        if peek != FRAME_MAGIC {
            return Err(DacantusError::Metadata(format!(
                "expected FRM1 or END1, got {:02x?}",
                peek
            )));
        }
        cur.take(4);
        let idx = cur.u32().ok_or(DacantusError::Truncated("frame_index"))? as usize;
        let len = cur.u16().ok_or(DacantusError::Truncated("frame_len"))? as usize;
        let payload = cur.take(len).ok_or(DacantusError::Truncated("frame_payload"))?;
        let got = cur.u32().ok_or(DacantusError::Truncated("frame_crc"))?;
        let want = crc32(payload);
        if got != want {
            return Err(DacantusError::ChecksumMismatch { expected: want, actual: got });
        }
        let off = idx * FRAME_PAYLOAD_MAX;
        if off + len > out.len() {
            return Err(DacantusError::Metadata(format!(
                "frame {} out of range (off={}, len={}, total={})",
                                                       idx, off, len, out.len()
            )));
        }
        out[off..off + len].copy_from_slice(payload);
        written += len;
    }

    if written != total_len {
        return Err(DacantusError::Metadata(format!(
            "wrote {} bytes, expected {}",
            written, total_len
        )));
    }

    let got = crc32(&out);
    if got != expected_crc {
        return Err(DacantusError::ChecksumMismatch { expected: expected_crc, actual: got });
    }

    Ok(EncodedFile { file_name, data: out })
}

struct ByteCursor<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> ByteCursor<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }
    fn peek(&self, n: usize) -> Option<&'a [u8]> {
        self.data.get(self.pos..self.pos + n)
    }
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        let s = self.data.get(self.pos..self.pos + n)?;
        self.pos += n;
        Some(s)
    }
    fn u8(&mut self) -> Option<u8> {
        self.take(1).map(|s| s[0])
    }
    fn u16(&mut self) -> Option<u16> {
        self.take(2).map(|s| u16::from_le_bytes([s[0], s[1]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.take(4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
    fn u64(&mut self) -> Option<u64> {
        self.take(8).map(|s| {
            u64::from_le_bytes([s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7]])
        })
    }
}
