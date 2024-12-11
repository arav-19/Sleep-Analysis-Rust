mod data;
mod analysis;
mod graph;
mod clustering;

const FILE_PATH: &str = "/Users/aravchawla/Desktop/DS210FINAL/rust-sleep-analysis/student_sleep_patterns.csv";

fn main() {
    let dataset = data::load_data(FILE_PATH);
    println!("Loaded {} records.", dataset.len());

    println!("Descriptive Statistics:");
    analysis::calculate_statistics(&dataset);

    println!("Building and Analyzing Graph...");
    let graph = graph::build_graph(&dataset);
    graph::analyze_graph(&graph);

    println!("Performing Clustering...");
    let clusters = clustering::k_means_clustering(&dataset, 3, 10);
    for (i, cluster) in clusters.iter().enumerate() {
        println!("Cluster {}: {} points", i + 1, cluster.points.len());
    }
}
