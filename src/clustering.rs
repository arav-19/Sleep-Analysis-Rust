use crate::data::StudentSleepData;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct Cluster {  
    pub centroid: Vec<f64>,
    pub points: Vec<usize>,
}

pub fn k_means_clustering(data: &[StudentSleepData], k: usize, iterations: usize) -> Vec<Cluster> {
    let mut rng = rand::thread_rng();

    let features: Vec<Vec<f64>> = data
        .iter()
        .map(|d| vec![d.sleep_duration, d.study_hours, d.screen_time])
        .collect();

    let mut centroids: Vec<Vec<f64>> = features
        .choose_multiple(&mut rng, k)
        .cloned()
        .collect();

    let mut clusters = vec![];

    for _ in 0..iterations {
        let mut new_clusters: Vec<Cluster> = vec![
            Cluster {
                centroid: vec![0.0; centroids[0].len()],
                points: vec![],
            };
            k
        ];

        for (i, feature) in features.iter().enumerate() {
            let closest_centroid = centroids
                .iter()
                .enumerate()
                .min_by(|(_, c1), (_, c2)| {
                    euclidean_distance(feature, c1)
                        .partial_cmp(&euclidean_distance(feature, c2))
                        .unwrap()
                })
                .unwrap()
                .0;

            new_clusters[closest_centroid].points.push(i);
        }

        for cluster in &mut new_clusters {
            let mut new_centroid = vec![0.0; centroids[0].len()];
            for &point_idx in &cluster.points {
                for (dim, value) in features[point_idx].iter().enumerate() {
                    new_centroid[dim] += value;
                }
            }
            if !cluster.points.is_empty() {
                for dim in 0..new_centroid.len() {
                    new_centroid[dim] /= cluster.points.len() as f64;
                }
            }
            cluster.centroid = new_centroid;
        }

        clusters = new_clusters;
        centroids = clusters.iter().map(|c| c.centroid.clone()).collect();
    }

    clusters
}

fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::StudentSleepData;

    #[test]
    fn test_k_means_clustering() {
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

        let clusters = k_means_clustering(&test_data, 2, 5);
        assert_eq!(clusters.len(), 2, "There should be 2 clusters");
        assert!(clusters[0].points.len() > 0, "Cluster 1 should not be empty");
    }
}
