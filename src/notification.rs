// use crate::alarm::Alarm;

use winrt_toast_reborn::content::text::TextPlacement;
use winrt_toast_reborn::{Header, Text, Toast, ToastManager};

// pub fn send_notification(alarm: &Alarm) {
//     let toast = Toast::new("Alarm App")
//         .title("Alarm Triggered!")
//         .text1(format!("Alarm time: {}", alarm.time.format("%I:%M %p")))
//         .duration(Duration::Long)
//         .action(Action::new("Dismiss", "Dismiss"))
//         .action(Action::new("Snooze", "Snooze"))
//         .sound(Some(winrt_notification::Sound::Default))
//         .show()
//         .expect("Failed to show toast");
// }

pub fn send_toast() {
    let manager = ToastManager::new("SarahY.AlarmApp");

    let mut toast = Toast::new();
    toast
        .text1("Hello, world!")
        .text2(Text::new("Body"))
        .text3(Text::new("Via SMS").with_placement(TextPlacement::Attribution));

    manager.show(&toast).expect("Failed to show toast");
}
