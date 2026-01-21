use std::collections::{HashMap, VecDeque};
use crate::essp_protocol::*;
use std::fs::OpenOptions;
use std::io::Write;

// Device types
pub const UNIT_TYPE_NV200: u8 = 0x06;  // Note validator
pub const UNIT_TYPE_SMART_HOPPER: u8 = 0x09;  // Coin device

/// Poll event structure
#[derive(Debug, Clone)]
pub struct PollEvent {
    pub event_code: u8,
    pub data1: u32,
    pub data2: u32,
    pub currency: [u8; 3],
}

impl PollEvent {
    pub fn new(event_code: u8, data1: u32) -> Self {
        Self {
            event_code,
            data1,
            data2: 0,
            currency: *b"BRL",
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.event_code];
        
        match self.event_code {
            EVENT_READ | EVENT_CREDIT | EVENT_COINS_VALUE_ADDED => {
                // These events take a channel number (byte) or value?
                // Standard says: 
                // READ: Byte 1 = Channel (0=unknown, >0=channel)
                // CREDIT: Byte 1 = Channel
                // COINS_VALUE_ADDED: Byte 1 = Channel? Or Value?
                // essp_device.py treats CREDIT data as "readed_value". 
                // But _read_note uses CHANNEL_BY_NOTE[note]. So it expects channel index.
                // For COINS_VALUE_ADDED, _read_coins returns value directly. 
                // So COINS_VALUE_ADDED might be sending value?
                // Protocol 6 COINS_VALUE_ADDED (0xBF) might be different.
                
                // Let's assume READ/CREDIT use Channel (u8)
                // We'll use data1 as channel/value
                if self.event_code == EVENT_COINS_VALUE_ADDED {
                    // Coin value added usually sends the value? Or channel?
                    // SDKs usually send channel. But let's check communication.py
                    // It logs "data1: {event_data.data1}".
                    // If my emulator sends 4-byte value in data1...
                    // Let's stick to variable length based on type.
                    
                    // For Protocol 6, COIN_CREDIT (0xDF) vs COINS_VALUE_ADDED (0xBF)
                    // 0xBF: "The value of the coins... Byte 1-4: Value".
                    // So 0xBF takes 4 bytes.
                    bytes.extend_from_slice(&self.data1.to_le_bytes()); 
                    bytes.extend_from_slice(&self.currency); // Protocol 6 often adds currency
                    // Wait, standard says 0xBF is 1 byte event + 4 bytes value? + 3 bytes CC?
                } else {
                    // READ/CREDIT: 1 byte channel
                    bytes.push(self.data1 as u8);
                }
            }
            EVENT_DISPENSING | EVENT_DISPENSED => {
                // Byte 1-4: Value
                bytes.extend_from_slice(&self.data1.to_le_bytes());
                // Protocol 6 might add currency?
                // Standard says DISPENSED (0xD2): "Byte 1-4: Value. Byte 5-7: CC"
                bytes.extend_from_slice(&self.currency);
            }
            EVENT_DISABLED | EVENT_RESET => {
                // No data
            }
            _ => {
                // Default fallback
            }
        }
        bytes
    }
}

/// Device state for a single validator
pub struct DeviceState {
    pub address: u8,
    pub unit_type: u8,
    pub firmware_version: String,
    pub protocol_version: u8,
    pub currency_code: [u8; 3],
    pub enabled: bool,
    pub payout_enabled: bool,
    pub inhibit_mask_low: u8,
    pub inhibit_mask_high: u8,
    pub balance: HashMap<u32, u16>,  // value in cents -> count
    pub event_queue: VecDeque<PollEvent>,
    pub channels: Vec<ChannelData>,
}

#[derive(Debug, Clone)]
pub struct ChannelData {
    pub security: u8,
    pub value: u32,  // In cents
    pub currency: [u8; 3],
}

impl DeviceState {
    pub fn new_note_device(address: u8) -> Self {
        let mut balance = HashMap::new();
        // Default note balances (in cents) for BRL
        balance.insert(200, 10);    // R$ 2.00
        balance.insert(500, 9);     // R$ 5.00
        balance.insert(1000, 10);   // R$ 10.00
        balance.insert(2000, 13);   // R$ 20.00
        balance.insert(5000, 5);    // R$ 50.00
        balance.insert(10000, 5);   // R$ 100.00
        balance.insert(20000, 5);   // R$ 200.00 (Added to match channels)

        let channels = vec![
            ChannelData { security: 2, value: 200, currency: *b"BRL" },
            ChannelData { security: 2, value: 500, currency: *b"BRL" },
            ChannelData { security: 2, value: 1000, currency: *b"BRL" },
            ChannelData { security: 2, value: 2000, currency: *b"BRL" },
            ChannelData { security: 2, value: 5000, currency: *b"BRL" },
            ChannelData { security: 2, value: 10000, currency: *b"BRL" },
            ChannelData { security: 2, value: 20000, currency: *b"BRL" },
        ];

        let mut event_queue = VecDeque::new();
        event_queue.push_back(PollEvent::new(EVENT_RESET, 0));

        Self {
            address,
            unit_type: UNIT_TYPE_NV200,
            firmware_version: "1.00".to_string(),
            protocol_version: 0x06,
            currency_code: *b"BRL",
            enabled: false,
            payout_enabled: false,
            inhibit_mask_low: 0x00,
            inhibit_mask_high: 0x00,
            balance,
            event_queue,
            channels,
        }
    }

    pub fn new_coin_device(address: u8) -> Self {
        let mut balance = HashMap::new();
        // Default coin balances (in cents)
        balance.insert(1, 15);     // 1¢
        balance.insert(5, 10);     // 5¢
        balance.insert(10, 3);     // 10¢
        balance.insert(25, 7);     // 25¢
        balance.insert(50, 5);     // 50¢
        balance.insert(100, 20);   // $1.00

        let channels = vec![
            ChannelData { security: 2, value: 1, currency: *b"USD" },
            ChannelData { security: 2, value: 5, currency: *b"USD" },
            ChannelData { security: 2, value: 10, currency: *b"USD" },
            ChannelData { security: 2, value: 25, currency: *b"USD" },
            ChannelData { security: 2, value: 50, currency: *b"USD" },
            ChannelData { security: 2, value: 100, currency: *b"USD" },
        ];

        let mut event_queue = VecDeque::new();
        event_queue.push_back(PollEvent::new(EVENT_RESET, 0));

        Self {
            address,
            unit_type: UNIT_TYPE_SMART_HOPPER,
            firmware_version: "1.00".to_string(),
            protocol_version: 0x06,
            currency_code: *b"USD",
            enabled: false,
            payout_enabled: false,
            inhibit_mask_low: 0x00,
            inhibit_mask_high: 0x00,
            balance,
            event_queue,
            channels,
        }
    }

// ...

    /// Handle command and return (status, response_data)
    pub fn handle_command(&mut self, cmd: &[u8]) -> (u8, Vec<u8>) {
        if cmd.is_empty() {
            return (RESPONSE_OK, vec![]);
        }

        // Log command for debugging
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("/tmp/emu_cmd.log") {
            let _ = writeln!(file, "CMD: 0x{:02X} (len={})", cmd[0], cmd.len());
        }

        match cmd[0] {
            CMD_SYNC => (RESPONSE_OK, vec![]),  // Just OK
            
            CMD_HOST_PROTOCOL => {
                // cmd[1] should be 0x06 for protocol version 6
                (RESPONSE_OK, vec![])  // Just OK
            }
            
            CMD_SETUP_REQUEST => (RESPONSE_OK, self.build_setup_response()),
            
            CMD_ENABLE => {
                self.enabled = true;
                (RESPONSE_OK, vec![])
            }
            
            CMD_DISABLE => {
                self.enabled = false;
                self.event_queue.push_back(PollEvent::new(EVENT_DISABLED, 0));
                (RESPONSE_OK, vec![])
            }
            
            CMD_SET_INHIBITS => {
                if cmd.len() >= 3 {
                    self.inhibit_mask_low = cmd[1];
                    self.inhibit_mask_high = cmd[2];
                }
                (RESPONSE_OK, vec![])
            }
            
            CMD_POLL => (RESPONSE_OK, self.build_poll_response()),
            
            CMD_GET_ALL_LEVELS => (RESPONSE_OK, self.build_levels_response()),
            
            // 0x41: Get Routing (often used instead of Get All Levels)
            0x41 => (RESPONSE_OK, self.build_levels_response()),

            // 0x35: Get Note Amount (Get Denomination Level)
            0x35 => (RESPONSE_OK, vec![0]), // Placeholder
            
            // 0x22: Observed from libessp during get_all_levels
            0x22 => (RESPONSE_OK, self.build_levels_response()),
            
            CMD_ENABLE_PAYOUT => {
                self.payout_enabled = true;
                (RESPONSE_OK, vec![])
            }
            
            CMD_PAYOUT => {
                if cmd.len() >= 8 {
                    let amount = u32::from_le_bytes([cmd[1], cmd[2], cmd[3], cmd[4]]);
                    self.handle_payout(amount);
                }
                (RESPONSE_OK, vec![])
            }
            
            CMD_SET_ROUTE => {
                // Route command for storage vs cashbox
                // For now, just acknowledge
                (RESPONSE_OK, vec![])
            }
            
            CMD_SETUP_ENCRYPTION => {
                // We reject encryption to force plain text
                (RESPONSE_COMMAND_NOT_KNOWN, vec![])
            }
            
            CMD_COIN_MECH_GLOBAL_INHIBIT => {
                // cmd[1] = 0 to inhibit, 1 to enable
                if cmd.len() >= 2 {
                    self.enabled = cmd[1] != 0;
                }
                (RESPONSE_OK, vec![])
            }
            
            _ => {
                eprintln!("Unknown command: 0x{:02X}", cmd[0]);
                // Reject unknown commands (including encrypted ones starting with 0x7E or similar)
                (RESPONSE_COMMAND_NOT_KNOWN, vec![])
            }
        }
    }

    fn build_setup_response(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        // Unit type (1 byte)
        response.push(self.unit_type);
        
        // Firmware version (4 bytes ASCII)
        let fw_str = format!("{:4}", self.firmware_version);
        let fw_bytes = fw_str.as_bytes();
        response.extend_from_slice(&fw_bytes[0..4]);
        
        // Country Code (3 bytes ASCII)
        response.extend_from_slice(&self.currency_code);

        // Value Multiplier (3 bytes) - Legacy/Unused in expanded
        response.extend_from_slice(&[0, 0, 0]);
        
        // Number of channels (1 byte)
        let num_channels = self.channels.len() as u8;
        response.push(num_channels);
        
        // Legacy Channel Values (n bytes)
        // Values divided by 100 (assuming multiplier 100)
        for channel in &self.channels {
            let legacy_val = (channel.value / 100) as u8;
            response.push(legacy_val);
        }

        // Legacy Channel Security (n bytes)
        for channel in &self.channels {
            response.push(channel.security);
        }

        // Real value multiplier (3 bytes)
        response.extend_from_slice(&[100, 0, 0]);
        
        // Protocol version (1 byte)
        response.push(self.protocol_version);

        // Expanded Channel data (Protocol 6+)
        // Reverting to "Padding" version (9 bytes): Currency(3), Pad(1), Value(4), Security(1)
        // This prevents Python crash, even if values are shifted.
        // We prioritize stability of connection.
        for channel in &self.channels {
            response.extend_from_slice(&channel.currency);
            response.push(0); // Null terminator/padding for Currency
            response.extend_from_slice(&channel.value.to_le_bytes());
            response.push(channel.security);
        }
        
        response
    }

    fn build_poll_response(&mut self) -> Vec<u8> {
        let mut response = Vec::new();
        
        // Number of events
        let event_count = std::cmp::min(self.event_queue.len(), 20) as u8;
        response.push(event_count);
        
        // Pop and encode events
        for _ in 0..event_count {
            if let Some(event) = self.event_queue.pop_front() {
                response.extend_from_slice(&event.to_bytes());
            }
        }
        
        // If no events, return disabled event
        if event_count == 0 {
            response[0] = 1;
            response.extend_from_slice(&PollEvent::new(EVENT_DISABLED, 0).to_bytes());
        }
        
        response
    }

    fn build_levels_response(&self) -> Vec<u8> {
        let mut response = Vec::new();
        
        // Number of denominations
        response.push(self.balance.len() as u8);
        
        // Each denomination: count(2b) + skip(1b) + value(4b) + currency(3b) = 10 bytes
        // Actually 9 bytes according to communication.py:212
        for (&value, &count) in &self.balance {
            response.extend_from_slice(&count.to_le_bytes());  // 2 bytes
            response.extend_from_slice(&value.to_le_bytes());  // 4 bytes
            response.extend_from_slice(&self.currency_code);    // 3 bytes
        }
        
        response
    }

    fn handle_payout(&mut self, amount: u32) {
        // Simulate payout by reducing balance
        // Generate dispensing and dispensed events
        self.event_queue.push_back(PollEvent::new(EVENT_DISPENSING, amount));
        self.event_queue.push_back(PollEvent::new(EVENT_DISPENSED, amount));
        
        // Actually reduce balance (simplified - just reduce largest denomination)
        if let Some((_value, count)) = self.balance.iter_mut().max_by_key(|(k, _)| *k) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    /// Insert a bill (for GUI simulation)
    pub fn insert_note(&mut self, value: u32) {
        if self.enabled {
            // Find channel for this value
            let mut channel_idx = 0;
            for (i, channel) in self.channels.iter().enumerate() {
                if channel.value == value {
                    channel_idx = (i + 1) as u32;
                    break;
                }
            }
            
            // If channel found, send events with channel index
            if channel_idx > 0 {
                self.event_queue.push_back(PollEvent::new(EVENT_READ, channel_idx));
                self.event_queue.push_back(PollEvent::new(EVENT_CREDIT, channel_idx));
                
                // Add to balance if it's a stored denomination
                if self.balance.contains_key(&value) {
                    *self.balance.get_mut(&value).unwrap() += 1;
                }
            }
        }
    }

    /// Insert coins (for GUI simulation)
    pub fn insert_coins(&mut self, value: u32) {
        if self.enabled {
            self.event_queue.push_back(PollEvent::new(EVENT_COINS_VALUE_ADDED, value));
            
            // Add to balance
            if self.balance.contains_key(&value) {
                *self.balance.get_mut(&value).unwrap() += 1;
            }
        }
    }
}
