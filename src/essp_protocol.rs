use anyhow::{anyhow, Result};
use crc::Crc;

// eSSP Protocol Constants
pub const STX: u8 = 0x7F;
pub const RESPONSE_OK: u8 = 0xF0;
pub const RESPONSE_ERROR: u8 = 0xFF;
pub const RESPONSE_KEY_NOT_SET: u8 = 0xFA;
pub const RESPONSE_COMMAND_NOT_KNOWN: u8 = 0xF2;

// Command codes
pub const CMD_SYNC: u8 = 0x11;
pub const CMD_SETUP_REQUEST: u8 = 0x05;
pub const CMD_HOST_PROTOCOL: u8 = 0x06;
pub const CMD_POLL: u8 = 0x07;
pub const CMD_ENABLE: u8 = 0x0A;
pub const CMD_DISABLE: u8 = 0x09;
pub const CMD_SET_INHIBITS: u8 = 0x02;
pub const CMD_GET_ALL_LEVELS: u8 = 0x3B;
pub const CMD_PAYOUT: u8 = 0x42;
pub const CMD_ENABLE_PAYOUT: u8 = 0x5C;
pub const CMD_SETUP_ENCRYPTION: u8 = 0x4A;
pub const CMD_SET_ROUTE: u8 = 0x3A;
pub const CMD_REJECT: u8 = 0x08;
pub const CMD_COIN_MECH_GLOBAL_INHIBIT: u8 = 0x49;

// Poll event codes (from constants.py)
pub const EVENT_RESET: u8 = 0xF1;
pub const EVENT_READ: u8 = 0xEF;
pub const EVENT_CREDIT: u8 = 0xEE;
pub const EVENT_REJECTING: u8 = 0xED;
pub const EVENT_REJECTED: u8 = 0xEC;
pub const EVENT_STACKING: u8 = 0xCC;
pub const EVENT_STACKED: u8 = 0xEB;
pub const EVENT_DISABLED: u8 = 0xE8;
pub const EVENT_DISPENSING: u8 = 0xDA;
pub const EVENT_DISPENSED: u8 = 0xD2;
pub const EVENT_JAMMED: u8 = 0xD5;
pub const EVENT_CASHBOX_REMOVED: u8 = 0xE3;
pub const EVENT_COINS_VALUE_ADDED: u8 = 0xBF;

// CRC-16 using polynomial 0x8005, init 0xFFFF, no reflection (matches eSSP spec)
const CRC_ESSP_ALGO: crc::Algorithm<u16> = crc::Algorithm {
    width: 16,
    poly: 0x8005,
    init: 0xFFFF,
    refin: false,
    refout: false,
    xorout: 0x0000,
    check: 0,
    residue: 0
};
const CRC_ESSP: Crc<u16> = Crc::<u16>::new(&CRC_ESSP_ALGO);

/// eSSP Packet structure
#[derive(Debug, Clone)]
pub struct EsspPacket {
    pub sequence: u8,
    pub data: Vec<u8>,
}

impl EsspPacket {
    pub fn new(sequence: u8, data: Vec<u8>) -> Self {
        Self { sequence, data }
    }

    /// Serialize packet to bytes with STX, sequence, length, data, and CRC
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::new();
        
        // Calculate stuffed data first
        let stuffed_data = stuff_bytes(&self.data);
        
        // STX
        packet.push(STX);
        
        // Sequence
        packet.push(self.sequence);

        // Length (data only, not including sequence or length byte itself)
        let length = stuffed_data.len();
        packet.push(length as u8);
        
        // Data (already stuffed)
        packet.extend_from_slice(&stuffed_data);
        
        // Calculate CRC on everything except STX
        let crc_data = &packet[1..];
        let crc = calculate_crc(crc_data);
        
        // Append CRC (little-endian)
        packet.push((crc & 0xFF) as u8);
        packet.push((crc >> 8) as u8);
        
        packet
    }

    /// Parse packet from bytes, returns packet and number of bytes consumed
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize)> {
        if data.len() < 5 {
            return Err(anyhow!("Packet too short"));
        }

        // Check STX
        if data[0] != STX {
            return Err(anyhow!("Invalid STX byte: expected 0x7F, got 0x{:02X}", data[0]));
        }

        // Byte 1 is Sequence
        let sequence = data[1];

        // Byte 2 is Length
        let length = data[2] as usize;
        
        // Total = STX(1) + SEQ(1) + LEN(1) + DATA(len) + CRC(2)
        let total_len = 1 + 1 + 1 + length + 2;

        if data.len() < total_len {
            return Err(anyhow!("Incomplete packet: need {}, have {}", total_len, data.len()));
        }

        // Verify CRC (calculated over SEQ + LEN + DATA)
        let crc_data = &data[1..total_len - 2];
        let expected_crc = calculate_crc(crc_data);
        let received_crc = u16::from_le_bytes([data[total_len - 2], data[total_len - 1]]);

        if expected_crc != received_crc {
            return Err(anyhow!(
                "CRC mismatch: expected 0x{:04X}, got 0x{:04X}",
                expected_crc,
                received_crc
            ));
        }
        
        // Extract and unstuff data
        let stuffed_data = &data[3..3 + length];
        let unstuffed_data = unstuff_bytes(stuffed_data)?;

        Ok((Self::new(sequence, unstuffed_data), total_len))
    }
}

/// Byte stuffing: 0x7F becomes 0x7F 0x7F
fn stuff_bytes(data: &[u8]) -> Vec<u8> {
    let mut stuffed = Vec::with_capacity(data.len());
    for &byte in data {
        stuffed.push(byte);
        if byte == STX {
            stuffed.push(STX);
        }
    }
    stuffed
}

/// Byte unstuffing: 0x7F 0x7F becomes 0x7F
fn unstuff_bytes(data: &[u8]) -> Result<Vec<u8>> {
    let mut unstuffed = Vec::new();
    let mut i = 0;
    while i < data.len() {
        let byte = data[i];
        unstuffed.push(byte);
        if byte == STX {
            if i + 1 < data.len() && data[i + 1] == STX {
                i += 1; // Skip the second 0x7F
            }
        }
        i += 1;
    }
    Ok(unstuffed)
}

/// Calculate CRC-16 for eSSP protocol
fn calculate_crc(data: &[u8]) -> u16 {
    CRC_ESSP.checksum(data)
}

/// Response builder helper
pub fn build_response(sequence: u8, status: u8, data: &[u8]) -> EsspPacket {
    let mut response_data = vec![status];
    response_data.extend_from_slice(data);
    EsspPacket::new(sequence, response_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_stuffing() {
        let data = vec![0x01, 0x7F, 0x02, 0x7F, 0x7F, 0x03];
        let stuffed = stuff_bytes(&data);
        assert_eq!(stuffed, vec![0x01, 0x7F, 0x7F, 0x02, 0x7F, 0x7F, 0x7F, 0x7F, 0x03]);
    }

    #[test]
    fn test_byte_unstuffing() {
        let stuffed = vec![0x01, 0x7F, 0x7F, 0x02, 0x7F, 0x7F, 0x7F, 0x7F, 0x03];
        let unstuffed = unstuff_bytes(&stuffed).unwrap();
        assert_eq!(unstuffed, vec![0x01, 0x7F, 0x02, 0x7F, 0x7F, 0x03]);
    }

    #[test]
    fn test_packet_roundtrip() {
        let original = EsspPacket::new(42, vec![CMD_SYNC]);
        let bytes = original.to_bytes();
        let (parsed, consumed) = EsspPacket::from_bytes(&bytes).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(parsed.sequence, original.sequence);
        assert_eq!(parsed.data, original.data);
    }

    #[test]
    fn test_packet_with_stx_in_data() {
        let original = EsspPacket::new(10, vec![0x05, 0x7F, 0x10, 0x7F]);
        let bytes = original.to_bytes();
        let (parsed, _) = EsspPacket::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.data, original.data);
    }
}
