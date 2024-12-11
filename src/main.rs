mod data;
mod analysis;

const FILE_PATH: &str = "/Users/aravchawla/Desktop/DS210FINAL/rust-sleep-analysis/student_sleep_patterns.csv";

fn main() {
    let dataset = data::load_data(FILE__PATH);
    println!("Loaded {} records.", dataset.len());

    println!("Descriptive Statistics:");
    analysis::calculate_statistics(&dataset);
}
