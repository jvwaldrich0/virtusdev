use anyhow::{Context, Result};
use nix::pty::{openpty, OpenptyResult};
use std::collections::HashMap;
use std::os::unix::io::{AsRawFd, RawFd};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::bill_emulator::DeviceState;
use crate::essp_protocol::*;

pub struct SerialBridge {
    master_fd: RawFd,
    slave_path: String,
    devices: Arc<Mutex<HashMap<u8, DeviceState>>>,
}

impl SerialBridge {
    pub fn new() -> Result<Self> {
        // Create pty pair
        let OpenptyResult { master, slave } = openpty(None, None)
            .context("Failed to create pty pair")?;

        // Get slave path using raw fd
        let master_fd = master.as_raw_fd();
        let slave_fd = slave.as_raw_fd();
        
        // Unlock and grant access to slave PTY
        unsafe {
            libc::grantpt(master_fd);
            libc::unlockpt(master_fd);
        }
        
        let slave_path = unsafe {
            let path_ptr = libc::ptsname(master_fd);
            if path_ptr.is_null() {
                return Err(anyhow::anyhow!("Failed to get slave pty path"));
            }
            std::ffi::CStr::from_ptr(path_ptr)
                .to_string_lossy()
                .into_owned()
        };

        // Close slave fd (we only need the path)
        // slave fd will be closed automatically when it goes out of scope (I/O safety)

        // Initialize devices
        let mut devices_map = HashMap::new();
        devices_map.insert(0x00, DeviceState::new_note_device(0x00));
        devices_map.insert(0x10, DeviceState::new_coin_device(0x10));

        println!("✓ Virtual serial port created: {}", slave_path);
        println!("  Configure payment system to use this port:");
        println!("  sudo nano /etc/cloudpark/payment_config.yml");
        println!("  Set: bill_validator_serial: {}", slave_path);

        // Extract raw fd before master goes out of scope
        let master_fd = master.as_raw_fd();
        std::mem::forget(master); // Prevent automatic close
        
        Ok(Self {
            master_fd,
            slave_path,
            devices: Arc::new(Mutex::new(devices_map)),
        })
    }

    pub fn slave_path(&self) -> &str {
        &self.slave_path
    }

    pub fn get_devices(&self) -> Arc<Mutex<HashMap<u8, DeviceState>>> {
        Arc::clone(&self.devices)
    }

    /// Start the serial communication loop
    pub fn run(&mut self) -> Result<()> {
        println!("🔄 Serial bridge running, waiting for connections...\n");

        let mut buffer = Vec::new();
        let mut read_buf = [0u8; 256];

        loop {
            // Read from serial port (non-blocking with timeout)
            match Self::read_with_timeout(self.master_fd, &mut read_buf, Duration::from_millis(100)) {
                Ok(n) if n > 0 => {
                    println!("[RX] Received {} bytes: {:02X?}", n, &read_buf[..n]);
                    buffer.extend_from_slice(&read_buf[..n]);
                    
                    // Try to parse packets from buffer
                    while let Ok((packet, consumed)) = EsspPacket::from_bytes(&buffer) {
                        println!("[PKT] Parsed packet: seq={}, data_len={}", packet.sequence, packet.data.len());
                        self.handle_packet(&packet)?;
                        buffer.drain(..consumed);
                    }
                    
                    // If buffer has data but couldn't parse, show it
                    if !buffer.is_empty() {
                        println!("[BUF] Unparsed buffer: {:02X?}", buffer);
                    }
                }
                Ok(_) => {
                    // Timeout or no data, continue
                    thread::sleep(Duration::from_millis(10));
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    thread::sleep(Duration::from_millis(100));
                }
            }
        }
    }

    fn read_with_timeout(fd: RawFd, buf: &mut [u8], timeout: Duration) -> Result<usize> {
        // Set up fd_set for select
        let mut read_fds = unsafe {
            let mut fds: libc::fd_set = std::mem::zeroed();
            libc::FD_ZERO(&mut fds);
            libc::FD_SET(fd, &mut fds);
            fds
        };

        let mut tv = libc::timeval {
            tv_sec: timeout.as_secs() as libc::time_t,
            tv_usec: timeout.subsec_micros() as libc::suseconds_t,
        };

        let result = unsafe {
            libc::select(
                fd + 1,
                &mut read_fds,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                &mut tv,
            )
        };

        if result < 0 {
            return Err(anyhow::anyhow!("select() failed"));
        }

        if result == 0 {
            // Timeout
            return Ok(0);
        }

        // Data available, read it
        let n = unsafe { libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len()) };
        if n < 0 {
            let errno = std::io::Error::last_os_error();
            // EIO (I/O error) typically means no reader is connected to the slave PTY
            // This is expected during startup before the payment system connects
            if errno.raw_os_error() == Some(5) {
                // EIO - no client connected yet, return 0 (no data) rather than error
                return Ok(0);
            }
            return Err(anyhow::anyhow!("read() failed: {}", errno));
        }

        Ok(n as usize)
    }

    fn handle_packet(&mut self, packet: &EsspPacket) -> Result<()> {
        // The actual device address is embedded in the data or we need to track it
        // For eSSP, the library (libessp.so) handles addressing via SSPAddress in SSP_COMMAND
        // Since we're emulating at the serial level, we need to handle both devices
        
        // For now, let's route based on command type (simplified)
        // In real implementation, address would be in a higher-level protocol wrapper
        // We'll handle both devices for all commands
        
        if packet.data.is_empty() {
            return Ok(());
        }

        let cmd_code = packet.data[0];
        
        // Determine device address (this is simplified - real protocol has addressing)
        // For testing, we'll respond with the note device (0x00) by default
        let device_addr = 0x00;

        let response = {
            let mut devices = self.devices.lock().unwrap();
            if let Some(device) = devices.get_mut(&device_addr) {
                let (status, response_data) = device.handle_command(&packet.data);
                let response = build_response(packet.sequence, status, &response_data);
                
                // Log the transaction
                Self::log_transaction(device_addr, cmd_code, &packet.data, &response_data);
                Some(response)
            } else {
                None
            }
        };
        
        if let Some(resp) = response {
            self.send_response(&resp)?;
        }

        Ok(())
    }

    fn send_response(&mut self, response: &EsspPacket) -> Result<()> {
        let bytes = response.to_bytes();
        let n = unsafe {
            libc::write(
                self.master_fd,
                bytes.as_ptr() as *const libc::c_void,
                bytes.len(),
            )
        };

        if n < 0 {
            return Err(anyhow::anyhow!("Failed to write response"));
        }

        Ok(())
    }

    fn log_transaction(device_addr: u8, cmd_code: u8, cmd_data: &[u8], response_data: &[u8]) {
        let cmd_name = match cmd_code {
            CMD_SYNC => "SYNC",
            CMD_SETUP_REQUEST => "SETUP_REQUEST",
            CMD_HOST_PROTOCOL => "HOST_PROTOCOL",
            CMD_POLL => "POLL",
            CMD_ENABLE => "ENABLE",
            CMD_DISABLE => "DISABLE",
            CMD_SET_INHIBITS => "SET_INHIBITS",
            CMD_GET_ALL_LEVELS => "GET_ALL_LEVELS",
            CMD_PAYOUT => "PAYOUT",
            CMD_ENABLE_PAYOUT => "ENABLE_PAYOUT",
            CMD_SETUP_ENCRYPTION => "SETUP_ENCRYPTION",
            CMD_SET_ROUTE => "SET_ROUTE",
            CMD_COIN_MECH_GLOBAL_INHIBIT => "COIN_MECH_GLOBAL_INHIBIT",
            _ => "UNKNOWN",
        };

        println!(
            "[0x{:02X}] {} (0x{:02X}) | CMD: {} bytes, RSP: {} bytes",
            device_addr,
            cmd_name,
            cmd_code,
            cmd_data.len(),
            response_data.len()
        );
    }
}

impl Drop for SerialBridge {
    fn drop(&mut self) {
        unsafe { libc::close(self.master_fd) };
        println!("\n✗ Serial bridge closed");
    }
}
