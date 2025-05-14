// mod alarm;
mod notification;
// mod scheduler;
mod utils;

use winrt_toast_reborn::register;

pub fn main() {
    register("SarahY.AlarmApp", "Alarm App", Some(&utils::APP_ICON)).expect("Failed to register app");

    notification::send_toast();
}
