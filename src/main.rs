use chrono::{Local, NaiveTime, Timelike};
use serde::Deserialize;
use std::fs;
use std::thread::sleep;
use std::time::Duration as StdDuration;
use winrt_notification::{Duration, Sound, Toast};

#[derive(Debug, Deserialize)]
struct WorkTime {
    start: String,
    end: String,
}

fn main() {
    let settings = get_settings();

    let start = &settings[2].start;
    let end = &settings[2].end;

    let parsed_start = parse_time(&start);
    let parsed_end = parse_time(&end);

    loop {
        let now = Local::now().time();

        if now < parsed_end && now > parsed_start {
            if now.minute() == 0 || now.minute() == 30 {
                send_alert();
            }

            let sleep_until_target = if now.minute() < 30 {
                (30 - now.minute()) as u64 * 60
            } else { 
                (60 - now.minute()) as u64 * 60
            };

            println!("sleep for {} seconds", sleep_until_target);
            sleep(StdDuration::from_secs(sleep_until_target));
        } else {
            break;
        }
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
    println!("alert");
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
