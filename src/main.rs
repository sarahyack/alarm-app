pub fn main() {
    use winrt_notification::{Duration, Toast};
    Toast::new(Toast::POWERSHELL_APP_ID)
        .title("Hello Rustling!")
        .text1("This is a simple example")
        .duration(Duration::Short)
        .show()
        .expect("Failed to show toast");
}
