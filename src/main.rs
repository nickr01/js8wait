use chrono::{Timelike, Utc};
use std::{thread, time};

fn main() {
    let now = Utc::now();

    let nanos_per_millis = 1000000;
    let millis_per_sec = 1000;
    let secs_modulus = 30;
    let millis_modulus = secs_modulus * millis_per_sec;

    let offset_millis = (now.nanosecond()/nanos_per_millis) % millis_modulus;
    println!(
        "Frame offset is {} milliseconds",
        offset_millis
    );

    let sleep_millis  = millis_modulus - offset_millis;

    if sleep_millis > 0 {
        let sleep_duration = time::Duration::from_millis(sleep_millis.into());
        println!(
            "Sleeping {} millis",
            sleep_millis
        );
        thread::sleep(sleep_duration);
    };
}
