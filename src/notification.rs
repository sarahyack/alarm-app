// use crate::alarm::Alarm;

use winrt_toast_reborn::content::text::TextPlacement;
use winrt_toast_reborn::content::action::Action;
use winrt_toast_reborn::{Header, Text, Toast, ToastManager};

pub fn send_toast() {
    let manager = ToastManager::new("SarahY.AlarmApp");

    let mut toast = Toast::new();
    toast
        .text1("Hello, world!")
        .text2(Text::new("Body"))
        .text3(Text::new("Via SMS").with_placement(TextPlacement::Attribution))
        .action(Action::new("Dismiss", "dismiss", ""));

    manager.show(&toast).expect("Failed to show toast");
}
