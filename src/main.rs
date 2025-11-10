// Blinko - A simple, lightweight Windows desktop health reminder
// Philosophy: Keep it simple, keep it stupid

mod config;
mod database;
mod detection;
mod reminder;
mod tray;
mod window;

use anyhow::Result;
use log::info;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    info!("Blinko starting...");

    // Load configuration
    let config = config::Config::load_or_default()?;
    info!("Configuration loaded");

    // Initialize database
    database::init()?;
    info!("Database initialized");

    // Create channels for communication
    let (reminder_tx, mut reminder_rx) = mpsc::channel(32);
    let (detection_tx, mut detection_rx) = mpsc::channel(32);

    // Start reminder timer
    let config_clone = config.clone();
    tokio::spawn(async move {
        reminder::start_timer(config_clone, reminder_tx).await;
    });

    // Start detection if enabled
    if config.blink_detection.enabled || config.posture_detection.enabled {
        let config_clone = config.clone();
        tokio::spawn(async move {
            detection::start_detection(config_clone, detection_tx).await;
        });
    }

    // Create system tray
    let mut tray = tray::SystemTray::new()?;
    info!("System tray created");

    // Create floating eye icon window
    let window = window::EyeWindow::new()?;
    info!("Eye window created");
    window.show();

    // Main event loop
    loop {
        tokio::select! {
            // Handle reminder events
            event = reminder_rx.recv() => {
                if let Some(reminder::ReminderEvent::TimeToRest) = event {
                    if let Err(e) = window.show_reminder().await {
                        log::error!("Failed to show reminder: {}", e);
                    }
                    // Record reminder
                    if let Err(e) = database::record_reminder("timer", Some("reminder")) {
                        log::error!("Failed to record reminder: {}", e);
                    }
                }
            }
            // Handle detection events
            event = detection_rx.recv() => {
                match event {
                    Some(detection::DetectionEvent::BlinkReminder) => {
                        if let Err(e) = window.show_blink_reminder().await {
                            log::error!("Failed to show blink reminder: {}", e);
                        }
                        if let Err(e) = database::record_reminder("blink", Some("alert")) {
                            log::error!("Failed to record blink alert: {}", e);
                        }
                    }
                    Some(detection::DetectionEvent::PostureReminder) => {
                        if let Err(e) = window.show_posture_reminder().await {
                            log::error!("Failed to show posture reminder: {}", e);
                        }
                        if let Err(e) = database::record_reminder("posture", Some("alert")) {
                            log::error!("Failed to record posture alert: {}", e);
                        }
                    }
                    None => break,
                }
            }
            // Handle tray events
            event = tray.wait_for_event() => {
                match event {
                    tray::TrayEvent::ShowWindow => {
                        window.show();
                    }
                    tray::TrayEvent::Quit => {
                        info!("Quitting...");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

