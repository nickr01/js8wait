use bwavfile::WaveReader;
use chrono::{Timelike, Utc};
use clap::Parser;
use rodio::{Decoder, OutputStream, Sink};
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(version, about = "Wait js8call frame", long_about = None)]
struct Opt {
    /// The WAV file to use
    #[arg(short, long, default_value_t = String::from("none"))]
    file: String,

//     /// The output device to use
//     #[arg(short, long, default_value_t = String::from("Default"))]
//     device: String,

    /// Slowest js8speed in test. Determines time modulus.
    #[arg(short, long, default_value_t = String::from("N"))]
    speed: String,
}

fn main() {
    let opt = Opt::parse();
    let file_name = opt.file;

    const NANOS_PER_MILLIS: u64 = 1000 * 1000;
    const MILLIS_PER_SEC: u64 = 1000;
    const NANOS_PER_SEC: u64 = MILLIS_PER_SEC * NANOS_PER_MILLIS;

    let modulus_secs: u64 = 15;
    let modulus_millis: u64 = modulus_secs * MILLIS_PER_SEC;
    let wav_offset_millis: u64 = {
        if &file_name != "none" {
            println!("Parsing {}", &file_name);
            let mut wavr = WaveReader::open(&file_name).unwrap();
            let format = wavr.format().unwrap();

            assert_eq!(format.sample_rate, 44100);
        //    assert_eq!(format.channel_count, 1);

            let sample_rate: u64 = format.sample_rate.into();

            let bext = wavr.broadcast_extension();
            let time_ref: u64 = bext.unwrap().unwrap().time_reference;

            let time_ref_millis: u64 = (time_ref * MILLIS_PER_SEC)/sample_rate;

            time_ref_millis % modulus_millis

        } else {
            println!("No WAV file");
            0
        }
    };

    println!(
        "Wave offset is {} milliseconds",
        wav_offset_millis
    );

    let now = Utc::now();
    let now_sec: u64 = now.second().into();
    let now_nanos_part: u64 = now.nanosecond().into();

    let now_nanos: u64 = now_sec * NANOS_PER_SEC + now_nanos_part;

    println!(
        "Now nanos is {} ",
        now_nanos
    );

    let now_offset_millis = (now_nanos/NANOS_PER_MILLIS) % modulus_millis;
    println!(
        "Now offset is {} milliseconds",
        now_offset_millis
    );

    let sleep_millis: u64
        = (modulus_millis + wav_offset_millis - now_offset_millis) % modulus_millis;

    println!(
        "Sleeping {} millis",
        sleep_millis
    );

    if sleep_millis > 0 {
        let sleep_duration = time::Duration::from_millis(sleep_millis.into());
        thread::sleep(sleep_duration);
    };

    if &file_name != "none" {
        // see if we can play it!
        println!("Playing {}", &file_name);

        // Get an output stream handle to the default physical sound device.
        // Note that no sound will be played if _stream is dropped
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        // Load a sound from a file, using a path relative to Cargo.toml
        let file = BufReader::new(File::open(&file_name).unwrap());

        // Decode that sound file into a source
        let source = Decoder::new(file).unwrap();
        sink.append(source);

        sink.sleep_until_end();
    }
    println!("Done");
}
