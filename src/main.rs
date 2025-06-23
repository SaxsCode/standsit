use chrono::{Local, NaiveTime};
use serde::Deserialize;
use std::fs;
use std::thread::sleep;
use std::time::Duration as StdDuration;
use winrt_notification::{Duration, Sound, Toast};

#[derive(Debug, Deserialize)]
struct WorkTime {
    interval: u64,
    start: String,
    end: String,
}

fn main() {
    let settings = get_settings();

    loop {
        let now = Local::now().time();

        let interval:u64 = if let Some(block_interval) = inside_block(&now, &settings) {
            send_alert();
            block_interval
        } else {
            match wait_for_next_block(&now, &settings) {
                Some(next_interval) => next_interval,
                None => break,
            }
        };

        wait_for_next_interval(interval);
    }
}

fn inside_block(now: &NaiveTime, settings: &[WorkTime]) -> Option<u64> {
    for block in settings {
        let start = parse_time(&block.start);
        let end = parse_time(&block.end);
        if now >= &start && now <= &end {
            return Some(block.interval);
        }
    }
    None
}

fn wait_for_next_interval(interval: u64) -> () {
    let sleep_in_seconds:u64 = &interval * 60;
    sleep(StdDuration::from_secs(sleep_in_seconds));
}

fn wait_for_next_block(time: &NaiveTime, settings: &[WorkTime]) -> Option<u64> {
    for block in settings {
        let start = parse_time(&block.start);
        let interval = block.interval;

        if time <= &start {
            let duration = start.signed_duration_since(*time);
            if duration > chrono::Duration::zero() {
                sleep(StdDuration::from_secs(duration.num_seconds() as u64));
            }
            return Some(interval);
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
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
