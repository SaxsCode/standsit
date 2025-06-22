use chrono::Local;
use chrono::NaiveTime;
use serde::Deserialize;
use std::fs;
use std::thread::sleep;
use std::time::Duration as StdDuration;
use winrt_notification::{Duration, Sound, Toast};

#[derive(Debug, Deserialize)]
struct WorkTime {
    title: String,
    range: Vec<String>,
    active: u8,
}

fn main() {
    let settings = get_settings();

    let start = &settings[0].range[0];
    let end = &settings[0].range[1];

    let parsed_start = parse_time(&start);
    let parsed_end = parse_time(&end);
    let now = Local::now().format("%H:%M");

    println!("{:?}", parsed_start);
    println!("{:?}", parsed_end);
    println!("{:?}", now);

    let mut second: u8 = 0;
    while second < 60 {
        if second == 10 {
            send_alert();
        }
        sleep(StdDuration::from_secs(1));
        second += 1;
    }
}

fn parse_time(time_string: &str) -> NaiveTime {
    
    return NaiveTime::parse_from_str(&time_string, "%H:%M").expect("Failed to parse time");
}

fn get_settings() -> Vec<WorkTime> {
    let file_content = fs::read_to_string("src/worktimes.json").expect("Failed to read");

    let response: Vec<WorkTime> = serde_json::from_str(&file_content).expect("Failed to parse");

    return response;
}

fn send_alert() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
