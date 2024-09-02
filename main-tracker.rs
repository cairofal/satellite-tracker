//Code developed with the help of AI to check which satellites are flying over a certain region
// Developed by Cairo Faleiros using AI resources. 

use reqwest::Error;
use serde::Deserialize;
use std::time::Duration;
use std::thread::sleep;

// Structure to deserialize the API
#[derive(Deserialize, Debug)]
struct Satellite {
    satname: String,
    satid: u64,
    lat: f64,
    lng: f64,
    alt: f64,
}

// Function to verify wich satelites are over the region. 
async fn check_satellites_over_region(lat: f64, lng: f64, radius: f64) -> Result<(), Error> {
    let url = format!("https://https://www.n2yo.com/database/positions?lat={}&lng={}&radius={}", lat, lng, radius); //The endpoint can be changed
    
    let response = reqwest::get(&url).await?;
    let satellites: Vec<Satellite> = response.json().await?;
    
    println!("Satélittes over the region:");
    for satellite in satellites {
        println!("{:?}", satellite);
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    let lat = -15.7801; // Ex: Brasilia lat
    let lng = -47.9292; // Ex: Brasilia long
    let radius = 500.0; // 500 km radius 
    
    loop {
        if let Err(e) = check_satellites_over_region(lat, lng, radius).await {
            eprintln!("Error obtaining data from satellites: {}", e);
        }
        sleep(Duration::from_secs(60)); // Wait 60 seconds before check again
    }
}
