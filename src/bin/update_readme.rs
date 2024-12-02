use std::fs;
use std::process::Command;

fn main() {
    println!("Running benchmarks...");

    // Run benchmarks
    let output = Command::new("cargo")
        .args(&["bench"])
        .output()
        .expect("Failed to run benchmarks");

    let results = String::from_utf8_lossy(&output.stdout);
    println!("Raw benchmark results:\n{}", results);

    // Parse benchmark results into a table
    let mut benchmarks_table =
        String::from("| Challenge       | Low         | Mean        | High        |\n");
    benchmarks_table.push_str("|-----------------|-------------|-------------|-------------|\n");

    results
        .lines()
        .filter(|line| line.contains("time:"))
        .for_each(|line| {
            // Extract the benchmark name and time ranges
            if let Some((name, times)) = line.split_once("time:") {
                let challenge = name.trim();
                let times = times.trim();

                // Clean up and parse times with units
                let mut parts = times.split_whitespace();
                let mut low =
                    parts.next().unwrap_or("").to_string() + " " + parts.next().unwrap_or("");
                let mean =
                    parts.next().unwrap_or("").to_string() + " " + parts.next().unwrap_or("");
                let mut high =
                    parts.next().unwrap_or("").to_string() + " " + parts.next().unwrap_or("");

                // Remove brackets from low and high
                low = low.trim_start_matches('[').to_string();
                high = high.trim_end_matches(']').to_string();

                benchmarks_table.push_str(&format!(
                    "| {:<15} | {:<11} | {:<11} | {:<11} |\n",
                    challenge, low, mean, high
                ));
            }
        });

    println!("Formatted benchmark table:\n{}", benchmarks_table);

    // Read README.md
    let readme_path = "README.md";
    let mut readme = fs::read_to_string(readme_path).expect("Failed to read README.md");

    // Replace benchmark placeholder
    let placeholder_start = "<!-- BENCHMARK RESULTS START -->";
    let placeholder_end = "<!-- BENCHMARK RESULTS END -->";
    if let (Some(start_idx), Some(end_idx)) =
        (readme.find(placeholder_start), readme.find(placeholder_end))
    {
        readme.replace_range(
            start_idx + placeholder_start.len()..end_idx,
            &format!("\n{}\n", benchmarks_table),
        );
        fs::write(readme_path, &readme).expect("Failed to write updated README.md");
        println!(
            "README.md updated successfully. Updated README.md:\n{}",
            readme
        );
    } else {
        println!("Placeholders not found in README.md.");
    }
}
