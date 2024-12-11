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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::StudentSleepData;

    #[test]
    fn test_calculate_statistics() {
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

        calculate_statistics(&test_data);
    }
}
