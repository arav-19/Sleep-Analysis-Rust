use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StudentSleepData {
    #[serde(rename = "Student_ID")]
    pub student_id: u32,
    #[serde(rename = "Age")]
    pub age: u8,
    #[serde(rename = "Gender")]
    pub gender: String,
    #[serde(rename = "University_Year")]
    pub university_year: String,
    #[serde(rename = "Sleep_Duration")]
    pub sleep_duration: f64,
    #[serde(rename = "Study_Hours")]
    pub study_hours: f64,
    #[serde(rename = "Screen_Time")]
    pub screen_time: f64,
    #[serde(rename = "Caffeine_Intake")]
    pub caffeine_intake: u8,
    #[serde(rename = "Physical_Activity")]
    pub physical_activity: u8,
    #[serde(rename = "Sleep_Quality")]
    pub sleep_quality: u8,
    #[serde(rename = "Weekday_Sleep_Start")]
    pub weekday_sleep_start: f64,
    #[serde(rename = "Weekend_Sleep_Start")]
    pub weekend_sleep_start: f64,
    #[serde(rename = "Weekday_Sleep_End")]
    pub weekday_sleep_end: f64,
    #[serde(rename = "Weekend_Sleep_End")]
    pub weekend_sleep_end: f64,
}

pub fn load_data(file_path: &str) -> Vec<StudentSleepData> {
    let mut reader = csv::Reader::from_path(file_path).expect("Failed to open file");
    reader
        .deserialize()
        .map(|result| result.expect("Error parsing record"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        const TEST_FILE: &str = "/Users/aravchawla/Desktop/DS210FINAL/rust-sleep-analysis/student_sleep_patterns.csv";

        let data = load_data(TEST_FILE);
        assert!(data.len() > 0, "Data loading failed or returned empty data");
        assert_eq!(data[0].age, 24, "First record's age should be 20"); 
    }
}
