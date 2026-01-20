mod device;

use device::{VirtualKeyboard, BAUDRATE, DEVICE_NAME, PRODUCT_ID, VENDOR_ID};
use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow, Box, Button, Entry, Label, Orientation, ScrolledWindow};
use glib::clone;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let app = Application::builder()
        .application_id("dev.virtus.barcode")
        .build();

    app.connect_activate(build_ui);
    app.run();
}

struct ScanRecord {
    barcode: String,
    duration: Duration,
}

struct AppState {
    device: Option<Arc<Mutex<VirtualKeyboard>>>,
    event_path: String,
    scan_history: Rc<RefCell<Vec<ScanRecord>>>,
}

fn build_ui(app: &Application) {
    // Try to create the virtual device
    let (device, event_path, error_msg) = match VirtualKeyboard::new() {
        Ok(device) => {
            let event_path = device.event_path().to_string();
            (Some(Arc::new(Mutex::new(device))), event_path, None)
        }
        Err(e) => {
            let error_msg = if e.to_string().contains("Permission denied") {
                "Permission denied: /dev/uinput\n\nPlease run with sudo:\n  sudo ./virtusdev\n\nOr add udev rule:\n  sudo usermod -a -G input $USER".to_string()
            } else {
                format!("Failed to create device: {}", e)
            };
            (None, String::new(), Some(error_msg))
        }
    };

    let window = ApplicationWindow::builder()
        .application(app)
        .title("VirtusDev - Virtual Barcode Scanner")
        .default_width(500)
        .default_height(600)
        .build();

    let main_box = Box::new(Orientation::Vertical, 10);
    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    if let Some(error) = error_msg {
        // Error view
        let error_label = Label::new(Some("⚠ Error"));
        error_label.add_css_class("error");
        let error_details = Label::new(Some(&error));
        error_details.set_wrap(true);
        
        main_box.append(&error_label);
        main_box.append(&error_details);
    } else {
        // Running view
        let state = Rc::new(RefCell::new(AppState {
            device,
            event_path: event_path.clone(),
            scan_history: Rc::new(RefCell::new(Vec::new())),
        }));

        // Status label
        let status_label = Label::new(Some("Status: ○ Running"));
        status_label.set_halign(gtk4::Align::Start);
        main_box.append(&status_label);

        // Device information
        let info_box = Box::new(Orientation::Vertical, 5);
        info_box.set_margin_top(10);
        
        let info_title = Label::new(Some("Device Information"));
        info_title.set_halign(gtk4::Align::Start);
        info_box.append(&info_title);
        
        info_box.append(&Label::new(Some(&format!("Name: {}", DEVICE_NAME))));
        info_box.append(&Label::new(Some(&format!("Event: {}", event_path))));
        info_box.append(&Label::new(Some(&format!("Baudrate: {} bps", BAUDRATE))));
        info_box.append(&Label::new(Some(&format!("Vendor ID: 0x{:04X}", VENDOR_ID))));
        info_box.append(&Label::new(Some(&format!("Product ID: 0x{:04X}", PRODUCT_ID))));
        
        main_box.append(&info_box);

        // Input section
        let input_box = Box::new(Orientation::Vertical, 10);
        input_box.set_margin_top(20);
        
        let entry = Entry::new();
        entry.set_placeholder_text(Some("Enter barcode..."));
        input_box.append(&entry);

        let scan_button = Button::with_label("SCAN");
        scan_button.add_css_class("suggested-action");
        input_box.append(&scan_button);

        main_box.append(&input_box);

        // History section
        let history_box = Box::new(Orientation::Vertical, 5);
        history_box.set_margin_top(20);
        
        let history_title = Label::new(Some("Recent Scans"));
        history_title.set_halign(gtk4::Align::Start);
        history_box.append(&history_title);
        
        let history_scroll = ScrolledWindow::builder()
            .hscrollbar_policy(gtk4::PolicyType::Never)
            .min_content_height(150)
            .build();
        
        let history_list = Box::new(Orientation::Vertical, 5);
        let no_scans_label = Label::new(Some("No scans yet"));
        no_scans_label.set_halign(gtk4::Align::Start);
        history_list.append(&no_scans_label);
        
        history_scroll.set_child(Some(&history_list));
        history_box.append(&history_scroll);
        main_box.append(&history_box);

        // Footer
        let footer = Label::new(Some("Made by jvwaldrich0"));
        footer.set_margin_top(20);
        footer.add_css_class("dim-label");
        main_box.append(&footer);

        // Connect scan button
        let state_clone = Rc::clone(&state);
        let entry_clone = entry.clone();
        let status_clone = status_label.clone();
        let history_clone = history_list.clone();
        
        scan_button.connect_clicked(clone!(@weak entry_clone as entry, @weak status_clone as status, @weak history_clone as history => move |_| {
            let barcode = entry.text().to_string();
            if barcode.is_empty() {
                return;
            }

            let state_ref = state_clone.borrow();
            if let Some(device) = &state_ref.device {
                status.set_text("Status: ● Scanning");
                
                let device_clone = Arc::clone(device);
                let barcode_for_device = barcode.clone();
                let barcode_for_display = barcode.clone();
                
                glib::spawn_future_local(clone!(@weak entry, @weak status, @weak history => async move {
                    let result: Result<Duration, anyhow::Error> = glib::spawn_future(async move {
                        let mut device = device_clone.lock().unwrap();
                        device.send_barcode(&barcode_for_device)
                    }).await.unwrap();

                    status.set_text("Status: ○ Running");
                    
                    match result {
                        Ok(duration) => {
                            let duration_ms = duration.as_secs_f64() * 1000.0;
                            let scan_label = Label::new(Some(&format!("• {} ({:.1}ms)", barcode_for_display, duration_ms)));
                            scan_label.set_halign(gtk4::Align::Start);
                            
                            // Clear "No scans yet" if present
                            if let Some(first_child) = history.first_child() {
                                if first_child.is::<Label>() {
                                    history.remove(&first_child);
                                }
                            }
                            
                            history.prepend(&scan_label);
                            entry.set_text("");
                        }
                        Err(e) => {
                            eprintln!("Scan failed: {}", e);
                        }
                    }
                }));
            }
        }));

        // Connect enter key
        entry.connect_activate(clone!(@weak scan_button => move |_| {
            scan_button.emit_clicked();
        }));
    }

    window.set_child(Some(&main_box));
    window.present();
}
