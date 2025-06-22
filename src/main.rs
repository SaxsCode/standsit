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

    let start_time = Local::now().time();
    let mut full_hour_mark = start_time.minute();
    let mut half_hour_mark = (full_hour_mark + 30) % 60;

    loop {
        let now = Local::now().time();
        let current_minute = now.minute();

        if inside_block(&now, &settings) {
            if current_minute == full_hour_mark || current_minute == half_hour_mark {
                send_alert();
            }
            wait_for_next_interval(current_minute);
        } else {
            if let Some(_start_time) = wait_for_next_block(&now, &settings) {
                full_hour_mark = _start_time.minute();
                half_hour_mark = (full_hour_mark + 30) % 60;
                continue; 
            } else {
                break;
            }
        }
    }
}

fn inside_block(now: &NaiveTime, settings: &[WorkTime]) -> bool {
    for block in settings {
        let start = parse_time(&block.start);
        let end = parse_time(&block.end);
        if now >= &start && now <= &end {
            return true;
        }
    }
    false
}

fn wait_for_next_interval(current_minute: u32) -> () {
    let sleep_until_target = if current_minute < 30 {
        (30 - current_minute) as u64 * 60
    } else {
        (60 - current_minute) as u64 * 60
    };

    println!("sleep for {} seconds", sleep_until_target);
    sleep(StdDuration::from_secs(sleep_until_target));
}

fn wait_for_next_block(time: &NaiveTime, settings: &[WorkTime]) -> Option<NaiveTime> {
    for block in settings {
        let start = parse_time(&block.start);
        if time <= &start {
            let duration = start.signed_duration_since(*time);
            if duration > chrono::Duration::zero() {
                sleep(StdDuration::from_secs(duration.num_seconds() as u64));
            }
            return Some(start);
        }
    }
    None
}

fn parse_time(time_string: &str) -> NaiveTime {
    NaiveTime::parse_from_str(&time_string, "%H:%M").expect("Failed to parse time")
}

fn get_settings() -> Vec<WorkTime> {
    let file_content = fs::read_to_string("src/worktimes.json").expect("Failed to read");
    serde_json::from_str(&file_content).expect("Failed to parse")
}

fn send_alert() -> () {
    println!("alert");
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
