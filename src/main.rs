use winrt_notification::{Duration, Sound, Toast};
use std::thread::sleep;
use std::time::Duration as StdDuration;

fn main() {
    let mut second:u8 = 0;
    while second < 60 {
        if second == 10 {
            send_alert();
        }
        sleep(StdDuration::from_secs(1));
        second+=1;
    }
}

fn send_alert() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
