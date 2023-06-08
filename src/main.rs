use std::time::Duration;

use notify_rust::Notification;
use tokio::time;

const DURATION: u64 = 30 * 60;
const STAND_UP: u64 = 60;

#[tokio::main]
async fn main() {
    loop {
        time::sleep(Duration::from_secs(DURATION)).await;
        display_notification().await;
    }
}

async fn display_notification() {
    Notification::new()
        .summary("Stand up!")
        .body("You've been sitting for 30 minutes.\nStand up a for at least 60 seconds\nYour future self will thank you")
        .show()
        .unwrap();
    time::sleep(Duration::from_secs(STAND_UP)).await;
    Notification::new()
        .summary("You may sit down now")
        .body("I hope you actually stood up!")
        .show()
        .unwrap();
}