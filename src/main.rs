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
    let full_hour_mark = start_time.minute();
    let half_hour_mark = (full_hour_mark + 30) % 60;

    loop {
        let now = Local::now().time();
        let current_minute = now.minute();

        if inside_block(&now, &settings) {
            if current_minute == full_hour_mark || current_minute == half_hour_mark {
                send_alert();
            }
            wait(current_minute);
        } else {
            break;
        }
    }
}

fn inside_block(now: &NaiveTime, settings: &[WorkTime]) -> bool {
    for block in settings {
        let start = parse_time(&block.start);
        let end = parse_time(&block.end);
        if now > &start && now < &end {
            return true;
        }
    }
    false
}

fn wait(current_minute: u32) -> () {
    let sleep_until_target = if current_minute < 30 {
        (30 - current_minute) as u64 * 60
    } else {
        (60 - current_minute) as u64 * 60
    };

    println!("sleep for {} seconds", sleep_until_target);
    sleep(StdDuration::from_secs(sleep_until_target));
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
