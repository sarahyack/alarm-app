// mod alarm;
mod notification;
// mod scheduler;

use std::path::Path;
use winrt_toast_reborn::register;

pub fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets/images")
        .join("alarm-app-icon.ico");
    register("SarahY.AlarmApp", "Alarm App", Some(&path)).expect("Failed to register app");

    notification::send_toast();
}
