use crate::data::StudentSleepData;

pub fn calculate_statistics(data: &[StudentSleepData]) {
    let sleep_durations: Vec<f64> = data.iter().map(|d| d.sleep_duration).collect();

    let mean = sleep_durations.iter().sum::<f64>() / sleep_durations.len() as f64;

    let mut sorted = sleep_durations.clone();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let median = if sorted.len() % 2 == 0 {
        (sorted[sorted.len() / 2 - 1] + sorted[sorted.len() / 2]) / 2.0
    } else {
        sorted[sorted.len() / 2]
    };

    println!("Mean Sleep Duration: {:.2}", mean);
    println!("Median Sleep Duration: {:.2}", median);
}
