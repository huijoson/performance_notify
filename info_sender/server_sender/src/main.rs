use actix_web::{HttpResponse};
use reqwest;
use server_disk::my_server_disk;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = my_server_disk::DiskInfo::get_server_data();
    // Send a POST request using reqwest
    match reqwest::Client::new()
        .post("http://localhost:8081/receive_message")
        .json(&data)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                HttpResponse::Ok().body("POST request sent successfully")
            } else {
                HttpResponse::InternalServerError().body("Failed to get a successful response")
            }
        }
        Err(err) => {
            eprintln!("Error sending POST request: {:?}", err);
            HttpResponse::InternalServerError().body("Error sending POST request")
        }
    };
    Ok(())
}
