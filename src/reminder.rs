// Simple reminder timer

use crate::config::Config;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

#[derive(Debug)]
pub enum ReminderEvent {
    TimeToRest,
}

pub async fn start_timer(config: Config, tx: mpsc::Sender<ReminderEvent>) {
    if !config.reminder.enabled {
        return;
    }

    let mut interval = interval(Duration::from_secs(config.reminder.interval_minutes * 60));

    loop {
        interval.tick().await;
        let _ = tx.send(ReminderEvent::TimeToRest).await;
    }
}

