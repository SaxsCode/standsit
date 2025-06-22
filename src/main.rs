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

    println!("{:?}", start);
    println!("{:?}", end);
          
    let mut second: u8 = 0;
    while second < 60 {
        if second == 10 {
            send_alert();
        }
        sleep(StdDuration::from_secs(1));
        second += 1;
    }
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
