use reqwest;
use server_disk;
use std::{collections::HashMap};
use std::time::Duration;
use tokio::time;

// Line Channel Access Token
const LINE_CHANNEL_ACCESS_TOKEN: &str = "8sXuO1vCYMOk5pXbxbp9CG5ActDUDzPX7ZVLeDFujbs";
const LINE_API_URL: &str = "https://notify-api.line.me/api/notify";

async fn send_line_notify(message: &str) {
    // request_URL
    let url = LINE_API_URL;

    // using HashMap create a form data
    let mut form_data = HashMap::new();
    form_data.insert("message", message);

    let client = reqwest::Client::new();
    let _ = client
        .post(url)
        .header(
            "Authorization",
            format!("Bearer {}", LINE_CHANNEL_ACCESS_TOKEN),
        )
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
        .await;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    loop {
        let result = server_disk::my_server_disk::DiskInfo::get_server_data();
        println!("result is {:?}", result);
        let message = format!(
            "hostname: {}, total: {}, used: {}, free: {}, percent: {}%",
            result.hostname, result.total, result.used, result.free, result.percent
        );
        // Call the send_line_notify function
        send_line_notify(&message).await;
        // Wait for 5 minutes
        time::sleep(Duration::from_secs(5)).await;
    }
}
