extern crate winrt_notification;
use winrt_notification::{Duration, Sound, Toast};

fn main() {
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Time to take a seat!")
        .sound(Some(Sound::SMS))
        .duration(Duration::Short)
        .show()
        .expect("unable to toast");
}
