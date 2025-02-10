use bwavfile::WaveReader;
use chrono::{Timelike, Utc};
use clap::Parser;
use cpal::traits::{DeviceTrait, HostTrait};
use rodio::{Decoder, OutputStream, Sink};
use serde::Serialize;
use std::{thread, time};
use std::fs::File;
use std::io::BufReader;

#[derive(
    clap::ValueEnum, Clone, Debug, Serialize,
)]
#[serde(rename_all = "lowercase")]
enum Speed {
    Slow,
    Normal,
    Fast,
    Turbo,
}

#[derive(Parser, Debug)]
#[command(version, about = "Wait js8call frame", long_about = None)]
struct Opt {
    /// The WAV file to use
    #[arg(short, long, default_value = "none")]
    file: String,

    /// The output device to use
    #[arg(short, long, default_value = "default")]
    device: String,

    /// Slowest js8speed in test. Determines time modulus.
    #[arg(short, long, default_value = "normal")]
    speed: Speed,
}

fn get_output_device(arg_device: String) -> cpal::Device {
    let host = cpal::default_host();
    if arg_device == "default" {
        println!("Using default output device.");
        host.default_output_device()
    } else {
        host.output_devices().unwrap()
            .find(|x| x.name().map(|y| y == arg_device).unwrap_or(false))
    }.expect("Failed to find output device.")
}

fn get_modulus(arg_speed: Speed) -> u32 {
    match arg_speed {
        Speed::Slow => 30,
        Speed::Normal => 15,
        Speed::Fast => 10,
        Speed::Turbo => 6,
    }
}

fn main() {
    let opt = Opt::parse();
    let file_name = opt.file;

    const NANOS_PER_MILLIS: u32 = 1000 * 1000;
    const MILLIS_PER_SEC: u32 = 1000;
    const NANOS_PER_SEC: u32 = MILLIS_PER_SEC * NANOS_PER_MILLIS;

    let output_device = get_output_device(opt.device);
    println!("Output device: {}", output_device.name().unwrap());

    let modulus_secs = get_modulus(opt.speed);
    println!("Modulus secs: {}", modulus_secs);

    let modulus_millis = modulus_secs * MILLIS_PER_SEC;

    let wav_offset_millis = {
        if &file_name != "none" {
            println!("Parsing {}", &file_name);
            let mut wavr = WaveReader::open(&file_name).unwrap();
            let format = wavr.format().unwrap();

            // force use of files which match js8 native input format - reduce transcode artefact
            assert_eq!(format.sample_rate, 48000);
            assert_eq!(format.channel_count, 1);
            assert_eq!(format.bits_per_sample, 16);

            let sample_rate = format.sample_rate;

            let bext = wavr.broadcast_extension().unwrap();

            let time_ref = bext.as_ref().unwrap().time_reference; // u64
            println!{"bext.time_reference {}", time_ref};
            // to be used if non-zero - it provides a way for DAW workflow to set offset
            // DAW should be rendered from a non-zero frame boundary
            // origination_time will then be ignored

            let time_ref_millis: u32 = ((time_ref as u32) * MILLIS_PER_SEC)/sample_rate;

            // Creation time in format `HH:MM:SS`.
            let origination_time = bext.unwrap().origination_time;
            println!{"bext.origination_time {}", origination_time};

            let origination_secs  = &origination_time[6..8];
            println!("Origination secs {}", origination_secs);

            let origination_secs: u32 = origination_secs.parse().expect("Cannot parse secs");
            let orig_ref_millis = origination_secs * MILLIS_PER_SEC;

            let offset = if time_ref_millis > 0 {
                println!("using bext.time_ref");
                time_ref_millis
            } else {
                println!("using bext.orig_time");
                orig_ref_millis
            };
            offset % (modulus_millis as u32)
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
    let now_sec = now.second();
    let now_nanos_part: u64 = now.nanosecond().into();

    let now_nanos: u64 = ((now_sec as u64) * (NANOS_PER_SEC as u64)) + now_nanos_part;

    println!(
        "Now nanos is {} ",
        now_nanos
    );

    let now_offset_millis = ((now_nanos/(NANOS_PER_MILLIS as u64)) as u32) % modulus_millis;
    println!(
        "Now offset is {} milliseconds",
        now_offset_millis
    );

    let sleep_millis
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
        let (_stream, stream_handle) = OutputStream::try_from_device(&output_device).unwrap();
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
