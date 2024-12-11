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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::StudentSleepData;

    #[test]
    fn test_build_graph() {
        let test_data = vec![
            StudentSleepData {
                student_id: 1,
                age: 20,
                gender: "Male".to_string(),
                university_year: "1st Year".to_string(),
                sleep_duration: 7.0,
                study_hours: 5.0,
                screen_time: 2.0,
                caffeine_intake: 2,
                physical_activity: 50,
                sleep_quality: 8,
                weekday_sleep_start: 22.0,
                weekend_sleep_start: 23.0,
                weekday_sleep_end: 6.0,
                weekend_sleep_end: 7.0,
            },
            StudentSleepData {
                student_id: 2,
                age: 21,
                gender: "Female".to_string(),
                university_year: "2nd Year".to_string(),
                sleep_duration: 6.5,
                study_hours: 6.0,
                screen_time: 3.0,
                caffeine_intake: 3,
                physical_activity: 40,
                sleep_quality: 7,
                weekday_sleep_start: 23.0,
                weekend_sleep_start: 0.0,
                weekday_sleep_end: 5.0,
                weekend_sleep_end: 7.0,
            },
        ];

        let graph = build_graph(&test_data);
        assert_eq!(graph.node_count(), 2, "Graph should contain two nodes");
    }
}
