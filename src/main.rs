mod data;
mod analysis;
mod graph;

const FILE_PATH: &str = "/Users/aravchawla/Desktop/DS210FINAL/rust-sleep-analysis/student_sleep_patterns.csv";

fn main() {
    let dataset = data::load_data(FILE_PATH);
    println!("Loaded {} records.", dataset.len());

    println!("Descriptive Statistics:");
    analysis::calculate_statistics(&dataset);

    println!("Building and Analyzing Graph...");
    let graph = graph::build_graph(&dataset);
    graph::analyze_graph(&graph);
}
