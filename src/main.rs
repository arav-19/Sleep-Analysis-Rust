mod data;

const file_path: &str = "/Users/<your-username>/Documents/rust-sleep-analysis/data/student_sleep_patterns.csv";

fn main() {
    let dataset = data::load_data(FILE_PATH);
    println!("Loaded {} records.", dataset.len());
}
