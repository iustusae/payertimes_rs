use chrono::Local;
use core::str;
use reqwest::Error;
use std::process::Command;

#[derive(serde::Deserialize)]
struct LocationInfo {
    loc: String, // Geolocation in "latitude,longitude" format
}

#[derive(serde::Deserialize)]
struct ApiResponse {
    code: i32,
    status: String,
    data: Data, // Adjusted to match the actual API response
}

#[derive(serde::Deserialize)]
struct Data {
    timings: Timings, // Prayer timings object
}

#[derive(serde::Deserialize)]
struct Timings {
    fajr: String,
    sunrise: String,
    dhuhr: String,
    asr: String,
    sunset: String,
    maghrib: String,
    isha: String,
    imsak: String,
    midnight: String,
    firstthird: String,
    lastthird: String,
}

fn main() -> Result<(), Error> {
    const IP_URL: &'static str = "https://api.ipify.org";
    const IP_INFO_URL: &'static str = "https://ipinfo.io";
    const API_URL: &'static str = "https://api.aladhan.com/v1/timings";

    // Get the public IP address
    let ip_command_out = Command::new("curl").arg(IP_URL).output().unwrap();
    let ip_addr = str::from_utf8(&ip_command_out.stdout).unwrap();
    println!("IP Address: {}", ip_addr);

    // Get geolocation information for the IP address
    let geolocation_info = Command::new("curl")
        .arg(format!("{}/{}", IP_INFO_URL, ip_addr))
        .output()
        .unwrap();

    // Parse the location information (latitude, longitude)
    let location_info: LocationInfo =
        serde_json::from_str(&str::from_utf8(&geolocation_info.stdout).unwrap()).unwrap();
    let loc_parts: Vec<&str> = location_info.loc.split(',').collect();
    let latitude: f32 = loc_parts[0].parse().unwrap();
    let longitude: f32 = loc_parts[1].parse().unwrap();

    // Get the current date in DD-MM-YYYY format
    let date = Local::now().format("%d-%m-%Y").to_string();
    println!("Date: {}", date);

    // Build the API URL with latitude, longitude, and date
    let url = format!(
        "{}/{}?latitude={}&longitude={}",
        API_URL, date, latitude, longitude
    );
    println!("Requesting URL: {}", url);

    // Make the API request to Aladhan API
    let response: ApiResponse = reqwest::blocking::get(&url)?.json()?;

    // Print the response code and timings
    println!("Response Code: {}", response.code);
    println!("Prayer Times:");
    println!("Fajr: {}", response.data.timings.fajr);
    println!("Sunrise: {}", response.data.timings.sunrise);
    println!("Dhuhr: {}", response.data.timings.dhuhr);
    println!("Asr: {}", response.data.timings.asr);
    println!("Sunset: {}", response.data.timings.sunset);
    println!("Maghrib: {}", response.data.timings.maghrib);
    println!("Isha: {}", response.data.timings.isha);
    println!("Imsak: {}", response.data.timings.imsak);
    println!("Midnight: {}", response.data.timings.midnight);
    println!("First third: {}", response.data.timings.firstthird);
    println!("Last third: {}", response.data.timings.lastthird);

    Ok(())
}
