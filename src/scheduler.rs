use chrono::Local;

loop {
    let alarms = alarm::load_alarms();
    let now = Local::now();

    for alarm in alarms {
        if alarm.enabled
        && alarm.time.hour() == now.hour()
        && alarm.time.minute() == now.minute()
        && alarm.repear.contains(&now.weekday())
        {
            notification::send_notification();
            audio::play_sound();
        }
    }

    sleep(Duration::from_secs(30));
}
