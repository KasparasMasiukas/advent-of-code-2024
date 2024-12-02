use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn touch_input_file(day: u32) -> Result<(), String> {
    let input_dir = format!("{}/input/2024", env::var("CARGO_MANIFEST_DIR").unwrap());
    let input_path = format!("{}/day{}.txt", input_dir, day);

    // Ensure the input directory exists
    fs::create_dir_all(&input_dir)
        .map_err(|e| format!("Failed to create input directory: {}", e))?;

    // Create the file if it doesn't exist
    if !Path::new(&input_path).exists() {
        File::create(&input_path)
            .map_err(|e| format!("Failed to create input file for day {}: {}", day, e))?;
        println!("Created empty input file for day {}.", day);
    } else {
        println!("Input file for day {} already exists.", day);
    }

    Ok(())
}

fn fetch_and_save_input(day: u32) -> Result<(), String> {
    let input_dir = format!("{}/input/2024", env::var("CARGO_MANIFEST_DIR").unwrap());
    let input_path = format!("{}/day{}.txt", input_dir, day);

    // Check if the file already has content
    if let Ok(content) = fs::read_to_string(&input_path) {
        if !content.trim().is_empty() {
            println!(
                "Input for day {} already exists and has content, skipping download.",
                day
            );
            return Ok(());
        }
    }

    let session_cookie = env::var("AOC_SESSION")
        .map_err(|_| "AOC_SESSION environment variable not set".to_string())?;
    let client = Client::new();
    let url = format!("https://adventofcode.com/2024/day/{}/input", day);

    // Fetch the input
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", session_cookie))
        .send()
        .map_err(|e| format!("Failed to send request for day {}: {}", day, e))?;

    // Check for HTTP success
    if !response.status().is_success() {
        return Err(format!(
            "Failed to fetch input for day {}: HTTP {}",
            day,
            response.status()
        ));
    }

    let text = response
        .text()
        .map_err(|e| format!("Failed to read response text for day {}: {}", day, e))?;

    // Save the input to the file
    fs::write(&input_path, text)
        .map_err(|e| format!("Failed to write input for day {}: {}", day, e))?;

    println!("Successfully downloaded input for day {}.", day);
    Ok(())
}

fn main() {
    for day in 1..=25 {
        if let Err(err) = touch_input_file(day) {
            println!("Failed to create input file for day {}: {}", day, err);
        }
    }

    // Second loop: Fetch and save inputs
    for day in 1..=25 {
        if let Err(err) = fetch_and_save_input(day) {
            println!("Stopping input downloads: {}", err);
            break;
        }
    }
}
