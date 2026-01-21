use virtusdev::serial_bridge::SerialBridge;
use std::io::{self, Write};
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("===========================================");
    println!("  VirtusDev - Bill Validator Emulator");
    println!("===========================================\n");

    // Create serial bridge
    let mut bridge = SerialBridge::new()?;
    
    println!("\n📌 Next steps:");
    println!("   1. Configure payment system:");
    println!("      sudo nano /etc/cloudpark/payment_config.yml");
    println!("      Set: bill_validator_serial: {}", bridge.slave_path());
    println!("   2. Start payment system:");
    println!("      cd /path/to/payment_totem/e06000e");
    println!("      sudo env/bin/python main.py\n");
    println!("Press Ctrl+C to stop the emulator\n");
    println!("===========================================\n");

    // Clone devices for control thread
    let devices = bridge.get_devices();

    // Spawn control thread for inserting bills/coins via stdin
    thread::spawn(move || {
        loop {
            print!("\n💵 Insert bill [1/2/5/10/20] or coin [0.01/0.05/0.10/0.25/0.50/1.00]: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF - stop reading stdin to prevent infinite loop
                    thread::sleep(std::time::Duration::from_secs(1));
                }
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }
                    
                    let value_cents = match input {
                        // Bills
                        "1" => Some(100),
                        "2" => Some(200),
                        "5" => Some(500),
                        "10" => Some(1000),
                        "20" => Some(2000),
                        // Coins
                        "0.01" => Some(1),
                        "0.05" => Some(5),
                        "0.10" => Some(10),
                        "0.25" => Some(25),
                        "0.50" => Some(50),
                        "1.00" | "1" => Some(100),
                        _ => None,
                    };

                    if let Some(value) = value_cents {
                        let mut devs = devices.lock().unwrap();
                        
                        if value >= 100 {
                            // It's a bill
                            if let Some(note_device) = devs.get_mut(&0x00) {
                                note_device.insert_note(value);
                                println!("✓ Inserted ${:.2} bill", value as f64 / 100.0);
                            }
                        } else {
                            // It's a coin
                            if let Some(coin_device) = devs.get_mut(&0x10) {
                                coin_device.insert_coins(value);
                                println!("✓ Inserted ${:.2} coin", value as f64 / 100.0);
                            }
                        }
                    } else {
                        println!("❌ Invalid value. Use: 1, 2, 5, 10, 20 (bills) or 0.01, 0.05, 0.10, 0.25, 0.50, 1.00 (coins)");
                    }
                }
                Err(error) => {
                    eprintln!("Error reading stdin: {}", error);
                    thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    });

    // Run serial bridge (blocking)
    bridge.run()?;

    Ok(())
}
