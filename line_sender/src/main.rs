use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use reqwest::{self};
use serde::Deserialize;
use std::collections::HashMap;

// access token

const LINE_CHANNEL_ACCESS_TOKEN: &str = "8sXuO1vCYMOk5pXbxbp9CG5ActDUDzPX7ZVLeDFujbs";
const LINE_API_URL: &str = "https://notify-api.line.me/api/notify";


// This struct represents state
#[derive(Debug)]
struct AppState {
    app_name: String,
}

#[derive(Deserialize, Debug)]
pub struct DiskInfo {
    pub size: i32,
    pub used: i32,
    pub avail: i32,
    pub percent: f32,
    pub mount_on: String,
    pub filesystem: String,
    pub hostname: String,
}

async fn send_line_notify(message: &str) -> Result<(), reqwest::Error> {

    let mut form_data = HashMap::new();
    form_data.insert("message", message);

    let client = reqwest::Client::new();
    let response = client
        .post(LINE_API_URL)
        .header(
            "Authorization",
            format!(
                "Bearer {}",
                LINE_CHANNEL_ACCESS_TOKEN,
            ),
        )
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&form_data)
        .send()
        .await?;

    println!("Response: {:?}", response);
    // add Error handling
    if let Err(e) = response.error_for_status() {
        return Err(e);
    }
    Ok(())
}

#[post("/receive_message")]
async fn receive_message(data: web::Json<DiskInfo>) -> impl Responder {
    let body_data = data.into_inner();

    // Add validation checks for the incoming data
    if body_data.size < 0 {
        return HttpResponse::BadRequest().body("Invalid disk information");
    } else {
        println!("I am running in the server disk crate");
        let message = format!(
            "Disk usage of {} is {}%",
            body_data.mount_on, body_data.percent
        );
        let _ = send_line_notify(&message).await;
    }

    // Print the body data
    println!("Received message: {:?}", body_data);
    // You can return a response, or modify it based on your requirements
    HttpResponse::Ok().body("Received message successfully")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger

    // Start the Actix Web server
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Line Sender"),
            }))
            .service(receive_message)
    })
    .bind("127.0.0.1:8081")? // Replace with your desired IP address and port
    .run()
    .await
}
