use fxhash::FxHashMap;
use reqwest::blocking::Client;
use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufWriter, Write};
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

const MAX_BLINKS: usize = 75;
const BIN_LUT_SIZE: usize = 100_000; // Increase to 10_000_000 for slower build but faster runtime

#[inline(always)]
const fn num_digits(n: u64) -> usize {
    match n {
        0..10 => 1,
        10..100 => 2,
        100..1_000 => 3,
        1_000..10_000 => 4,
        10_000..100_000 => 5,
        100_000..1_000_000 => 6,
        1_000_000..10_000_000 => 7,
        10_000_000..100_000_000 => 8,
        100_000_000..1_000_000_000 => 9,
        1_000_000_000..10_000_000_000 => 10,
        10_000_000_000..100_000_000_000 => 11,
        100_000_000_000..1_000_000_000_000 => 12,
        1_000_000_000_000..10_000_000_000_000 => 13,
        10_000_000_000_000..100_000_000_000_000 => 14,
        100_000_000_000_000..1_000_000_000_000_000 => 15,
        1_000_000_000_000_000..10_000_000_000_000_000 => 16,
        10_000_000_000_000_000..100_000_000_000_000_000 => 17,
        _ => 18, // ..u64::MAX
    }
}

/// Get a divisor for splitting a number in half.
#[inline(always)]
const fn half_divisor(n: u64) -> u64 {
    match n {
        0..1_000 => 10,                                                // Half of 2 digits
        1_000..100_000 => 100,                                         // Half of 4 digits
        100_000..10_000_000 => 1_000,                                  // Half of 6 digits
        10_000_000..1_000_000_000 => 10_000,                           // Half of 8 digits
        1_000_000_000..100_000_000_000 => 100_000,                     // Half of 10 digits
        100_000_000_000..10_000_000_000_000 => 1_000_000,              // Half of 12 digits
        10_000_000_000_000..1_000_000_000_000_000 => 10_000_000,       // Half of 14 digits
        1_000_000_000_000_000..100_000_000_000_000_000 => 100_000_000, // Half of 16 digits
        _ => 1_000_000_000,                                            // Half of 18 digits
    }
}

#[inline(always)]
const fn split_number(n: u64) -> (u64, u64) {
    let divisor = half_divisor(n);
    let left = n / divisor;
    let right = n % divisor;
    (left, right)
}

fn count_stones_with_cache(
    blink: usize,
    stone_num: u64,
    cache: &mut FxHashMap<(u8, u64), u64>,
) -> u64 {
    // Define the key for caching
    let key = (blink as u8, stone_num);

    // Check if the result is already cached
    if let Some(&cached_count) = cache.get(&key) {
        return cached_count;
    }

    // Base case: If we've reached the maximum blink level
    if blink == MAX_BLINKS - 1 {
        let digits = num_digits(stone_num);
        let count = 2 - (digits as u64 % 2); // 1 stone if odd digits, 2 if even digits (split)
        return count;
    }

    // Recursive case
    if stone_num == 0 {
        // Rule 1: Replace 0 with 1
        let count = count_stones_with_cache(blink + 1, 1, cache);
        cache.insert(key, count);
        return count;
    }
    let digits = num_digits(stone_num);
    let count = if digits % 2 == 0 {
        // Rule 2: Even number of digits, split into two stones
        let (left, right) = split_number(stone_num);
        count_stones_with_cache(blink + 1, left, cache)
            + count_stones_with_cache(blink + 1, right, cache)
    } else {
        // Rule 3: Odd number of digits, replace with stone_num * 2024
        count_stones_with_cache(blink + 1, stone_num * 2024, cache)
    };

    // Insert the computed count into the cache
    cache.insert(key, count);

    // Return the computed count
    count
}

/// Computes the LUT for Day 11.
///
/// # Returns
/// A heap-allocated 2D `Vec<Vec<u64>>` representing the LUT.
fn compute_day11_lut() -> (Vec<u64>, Vec<u64>) {
    let mut lut1: Vec<u64> = Vec::with_capacity(BIN_LUT_SIZE);
    let mut lut2: Vec<u64> = Vec::with_capacity(BIN_LUT_SIZE);
    let mut cache = FxHashMap::default();

    // lut1 is for the last 25 blinks
    let blink = MAX_BLINKS - 25;
    for i in 0..BIN_LUT_SIZE {
        if i % (BIN_LUT_SIZE / 100) == 0 {
            println!("cargo:info=LUT 1: Processing stone number: {}", i);
        }
        let sum = count_stones_with_cache(blink, i as u64, &mut cache);
        lut1.push(sum);
    }

    // lut2 is for max blinks
    for i in 0..BIN_LUT_SIZE {
        if i % (BIN_LUT_SIZE / 100) == 0 {
            println!("cargo:info=LUT 2: Processing stone number: {}", i);
        }
        let sum = count_stones_with_cache(0, i as u64, &mut cache);
        lut2.push(sum);
    }

    (lut1, lut2)
}

fn build_day11_lut() {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let lut1_path = Path::new(&out_dir).join("day11lut1.bin"); // For 25 blinks
    let lut2_path = Path::new(&out_dir).join("day11lut2.bin"); // For 75 blinks

    if lut1_path.exists() && lut2_path.exists() {
        println!("cargo:warning=LUT binaries already exist at {:?}", out_dir);
        return;
    }

    println!(
        "cargo:warning=Building LUT binaries at {:?}... This may take a while...",
        out_dir
    );

    let (lut1, lut2) = compute_day11_lut();

    {
        let file = File::create(&lut1_path).expect("Failed to create LUT1 binary file");
        let mut writer = BufWriter::new(file);
        for &num in &lut1 {
            writer
                .write_all(&num.to_le_bytes())
                .expect("Failed to write LUT1 data");
        }
    }

    {
        let file = File::create(&lut2_path).expect("Failed to create LUT2 binary file");
        let mut writer = BufWriter::new(file);
        for &num in &lut2 {
            writer
                .write_all(&num.to_le_bytes())
                .expect("Failed to write LUT2 data");
        }
    }
    println!("cargo:warning=Built LUT binaries at {:?}", out_dir);

    println!("cargo:rerun-if-changed=build.rs");
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

    build_day11_lut()
}
