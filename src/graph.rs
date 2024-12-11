use petgraph::graph::Graph;
use crate::data::StudentSleepData;

pub fn build_graph(data: &[StudentSleepData]) -> Graph<String, f64> {
    let mut graph = Graph::<String, f64>::new();

    let mut node_indices = std::collections::HashMap::new();
    for record in data {
        let year = &record.university_year;
        if !node_indices.contains_key(year) {
            let index = graph.add_node(year.clone());
            node_indices.insert(year.clone(), index);
        }
    }

    for (year1, index1) in &node_indices {
        for (year2, index2) in &node_indices {
            if year1 != year2 {
                let avg_sleep1: f64 = data
                    .iter()
                    .filter(|d| &d.university_year == year1)
                    .map(|d| d.sleep_duration)
                    .sum::<f64>()
                    / data.iter().filter(|d| &d.university_year == year1).count() as f64;

                let avg_sleep2: f64 = data
                    .iter()
                    .filter(|d| &d.university_year == year2)
                    .map(|d| d.sleep_duration)
                    .sum::<f64>()
                    / data.iter().filter(|d| &d.university_year == year2).count() as f64;

                let similarity = 1.0 / (1.0 + (avg_sleep1 - avg_sleep2).abs());
                graph.add_edge(*index1, *index2, similarity);
            }
        }
    }

    graph
}

pub fn analyze_graph(graph: &Graph<String, f64>) {
    println!("--- Graph Analysis ---");
    println!("Number of nodes: {}", graph.node_count());
    println!("Number of edges: {}", graph.edge_count());
}
