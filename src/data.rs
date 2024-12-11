use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StudentSleepData {
    pub student_id: u32,
    pub age: u8,
    pub gender: String,
    pub university_year: String,
    pub sleep_duration: f64,
    pub study_hours: f64,
    pub screen_time: f64,
    pub caffeine_intake: u8,
    pub physical_activity: u8,
    pub sleep_quality: u8,
    pub weekday_sleep_start: f64,
    pub weekend_sleep_start: f64,
    pub weekday_sleep_end: f64,
    pub weekend_sleep_end: f64,
}

pub fn load_data(file_path: &str) -> Vec<StudentSleepData> {
    let mut reader = csv::Reader::from_path(file_path).expect("Failed to open file");
    reader
        .deserialize()
        .map(|result| result.expect("Error parsing record"))
        .collect()
}