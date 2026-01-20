use anyhow::{Context, Result};
use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, EventType, InputEvent, Key,
};
use std::thread;
use std::time::{Duration, Instant};

// Device configuration constants
pub const BAUDRATE: u32 = 115200;
pub const DEVICE_NAME: &str = "Virtual Keyboard 115200";
pub const VENDOR_ID: u16 = 0x1234;
pub const PRODUCT_ID: u16 = 0x5678;
pub const DEVICE_VERSION: u16 = 0x0001;

const BITS_PER_BYTE: u32 = 10;
const BIT_TIME_US: u32 = 1_000_000 / BAUDRATE;
const CHAR_TIME_US: u32 = BIT_TIME_US * BITS_PER_BYTE;
const KEY_PRESS_DELAY: Duration = Duration::from_micros(CHAR_TIME_US as u64);
const KEY_RELEASE_DELAY: Duration = Duration::from_micros(CHAR_TIME_US as u64);
const INTER_KEY_DELAY: Duration = Duration::from_micros((CHAR_TIME_US * 2) as u64);

pub struct VirtualKeyboard {
    device: VirtualDevice,
    event_path: String,
}

impl VirtualKeyboard {
    pub fn new() -> Result<Self> {
        // Create attribute set for all keys
        let mut keys = AttributeSet::<Key>::new();
        for code in 0..256 {
            keys.insert(Key::new(code));
        }

        let mut device = VirtualDeviceBuilder::new()?
            .name(DEVICE_NAME)
            .input_id(evdev::InputId::new(
                evdev::BusType::BUS_USB,
                VENDOR_ID,
                PRODUCT_ID,
                DEVICE_VERSION,
            ))
            .with_keys(&keys)?
            .build()
            .context("Failed to create virtual device")?;

        // Give the system time to register the device
        thread::sleep(Duration::from_millis(100));

        // Get the event path
        let event_path = device
            .enumerate_dev_nodes_blocking()?
            .next()
            .and_then(|path| path.ok())
            .and_then(|path| path.to_str().map(|s| s.to_string()))
            .unwrap_or_else(|| "Unknown".to_string());

        Ok(Self {
            device,
            event_path,
        })
    }

    pub fn event_path(&self) -> &str {
        &self.event_path
    }

    pub fn send_barcode(&mut self, barcode: &str) -> Result<Duration> {
        let start = Instant::now();

        for c in barcode.chars() {
            self.send_key(c)?;
        }

        // Send Enter key
        self.send_key('\n')?;

        Ok(start.elapsed())
    }

    fn send_key(&mut self, c: char) -> Result<()> {
        let keycode = get_keycode(c);
        if keycode.is_none() {
            return Ok(()); // Skip unsupported characters
        }

        let key = keycode.unwrap();
        let needs_shift = needs_shift(c);

        if needs_shift {
            self.emit(Key::KEY_LEFTSHIFT, 1)?;
            thread::sleep(KEY_PRESS_DELAY);
        }

        self.emit(key, 1)?;
        thread::sleep(KEY_PRESS_DELAY);

        self.emit(key, 0)?;
        thread::sleep(KEY_RELEASE_DELAY);

        if needs_shift {
            self.emit(Key::KEY_LEFTSHIFT, 0)?;
            thread::sleep(KEY_RELEASE_DELAY);
        }

        thread::sleep(INTER_KEY_DELAY);
        Ok(())
    }

    fn emit(&mut self, key: Key, value: i32) -> Result<()> {
        let events = [
            InputEvent::new(EventType::KEY, key.code(), value),
            InputEvent::new(EventType::SYNCHRONIZATION, 0, 0),
        ];
        self.device
            .emit(&events)
            .context("Failed to emit key event")?;
        Ok(())
    }
}

fn get_keycode(c: char) -> Option<Key> {
    match c {
        'a'..='z' => {
            let offset = (c as u8) - b'a';
            Some(Key::new(Key::KEY_A.code() + offset as u16))
        }
        'A'..='Z' => {
            let offset = (c as u8) - b'A';
            Some(Key::new(Key::KEY_A.code() + offset as u16))
        }
        '0'..='9' => {
            let offset = (c as u8) - b'0';
            Some(Key::new(Key::KEY_1.code() + offset as u16))
        }
        ' ' => Some(Key::KEY_SPACE),
        '\n' => Some(Key::KEY_ENTER),
        '\t' => Some(Key::KEY_TAB),
        '-' | '_' => Some(Key::KEY_MINUS),
        '=' | '+' => Some(Key::KEY_EQUAL),
        '[' | '{' => Some(Key::KEY_LEFTBRACE),
        ']' | '}' => Some(Key::KEY_RIGHTBRACE),
        ';' | ':' => Some(Key::KEY_SEMICOLON),
        '\'' | '"' => Some(Key::KEY_APOSTROPHE),
        '`' | '~' => Some(Key::KEY_GRAVE),
        '\\' | '|' => Some(Key::KEY_BACKSLASH),
        ',' | '<' => Some(Key::KEY_COMMA),
        '.' | '>' => Some(Key::KEY_DOT),
        '/' | '?' => Some(Key::KEY_SLASH),
        _ => None,
    }
}

fn needs_shift(c: char) -> bool {
    matches!(
        c,
        'A'..='Z'
            | '!'
            | '@'
            | '#'
            | '$'
            | '%'
            | '^'
            | '&'
            | '*'
            | '('
            | ')'
            | '_'
            | '+'
            | '{'
            | '}'
            | '|'
            | ':'
            | '"'
            | '<'
            | '>'
            | '?'
            | '~'
    )
}
