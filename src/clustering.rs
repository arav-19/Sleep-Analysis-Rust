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
