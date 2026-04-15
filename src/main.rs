// src/main.rs

// change hound -> cpal

use std::f32;

// needed for traits
use chrono::Timelike;
use clap::Parser;
use cpal::*;
use cpal::traits::{HostTrait, DeviceTrait, StreamTrait};
// use dasp_sample::{I48,U48};

const NANOS_PER_MILLIS: u32 = 1000 * 1000;
const MILLIS_PER_SEC: u32 = 1000;
const NANOS_PER_SEC: u32 = MILLIS_PER_SEC * NANOS_PER_MILLIS;

#[derive(
    clap::ValueEnum, Clone, Debug, serde::Serialize,
)]
#[serde(rename_all = "lowercase")]
enum Speed {
    Slow,
    Normal,
    Fast,
    Turbo,
}

#[derive(clap::Parser, Debug)]
#[command(version, about = "Wait js8call frame", long_about = None)]
struct Opt {
    /// The WAV file to use - can be none to allow just to wait for time modulus
    #[arg(short, long, default_value = "none")]
    input_file: Option<String>,

    /// The out out_device to use- can be none to allow just to wait for time modulus
    #[arg(short, long, default_value = "default")]
    out_device: Option<String>,

    /// Slowest js8speed in test. Determines time modulus.
    #[arg(short, long, default_value = "normal")]
    speed: Speed,
}

fn main() {
    let opt = Opt::parse();
    // let mut signal: Vec<f32> = Vec::new();

    // 1 Establish the out device and it's required format
    let host = cpal::default_host();

    let out_device = if let Some(out_device) = opt.out_device {
        if out_device != "default" {
            let id = &out_device.parse().expect("failed to parse out_device id");
            host.device_by_id(id)
        } else {
            // should fix to return a None
            host.default_output_device()
        }
    } else {
        host.default_output_device()
    }
    .expect("failed to establish out_device");
    println!("Output_device: {}", out_device.id().unwrap());


    let out_config = out_device.default_output_config().unwrap();
    println!("Default out config: {out_config:?}");

    // 2 Read the input file and establish the modulus
    let mut input_wav = if let Some(input_file) = opt.input_file {
        hound::WavReader::open(input_file).unwrap()
    } else {
        hound::WavReader::open("test.wav").unwrap()
    };

    let out_samples = samples_from_wav(&mut input_wav, &out_config.clone().into());

    let modulus_secs = get_modulus(opt.speed);
    println!("Modulus secs: {}", modulus_secs);

    let modulus_millis = modulus_secs * MILLIS_PER_SEC;

    let wav_offset_millis = {
        // if &input_file != "none" {
        // println!("Parsing {}", &file_name);
        // let mut wavr = WaveReader::open(&file_name).expect("Cannot build WaveReader");
        // let format = wavr.format().expect("Cannot get WaveReader format");

        // // force use of files which match js8 native input format - reduce transcode artefact
        // // assert_eq!(format.sample_rate, 48000);
        // // assert_eq!(format.channel_count, 1);
        // // assert_eq!(format.bits_per_sample, 16);

        // let sample_rate = format.sample_rate;

        // let bext = wavr.broadcast_extension().expect("Cannot read broadcast extension");

        // // let time_ref = bext.as_ref().expect("Cannot read time reference").time_reference; // u64
        // let time_ref: u32 = 0;
        // println!{"bext.time_reference {}", time_ref};
        // // to be used if non-zero - it provides a way for DAW workflow to set offset
        // // DAW should be rendered from a non-zero frame boundary
        // // origination_time will then be ignored

        // let time_ref_millis: u32 = ((time_ref as u32) * MILLIS_PER_SEC)/sample_rate;

        // // Creation time in format `HH:MM:SS`.
        // let origination_time = bext.expect("Cannot read origination time").origination_time;
        // println!{"bext.origination_time {}", origination_time};

        // let origination_secs  = &origination_time[6..8];
        // println!("Origination secs {}", origination_secs);

        // let origination_secs: u32 = origination_secs.parse().expect("Cannot parse secs");
        // let orig_ref_millis = origination_secs * MILLIS_PER_SEC;

        // let offset = if time_ref_millis > 0 {
        //     println!("using bext.time_ref");
        //     time_ref_millis
        // } else {
        //     println!("using bext.orig_time");
        //     orig_ref_millis
        // };
        //     let offset = 0;
        //     offset % (modulus_millis as u32)
        // } else {
        //     println!("No WAV file");
            0
    };

    // 3 Sleep
    println!(
        "Wave offset is {} milliseconds",
        wav_offset_millis
    );

    let now = chrono::Utc::now();
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
        let sleep_duration = std::time::Duration::from_millis(sleep_millis.into());
        std::thread::sleep(sleep_duration);
    };

    // 4 Play
    match out_config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::I16 => run::<i16>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::I24 => run::<I24>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::I32 => run::<i32>(out_samples, &out_device, out_config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::I64 => run::<i64>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::U8 => run::<u8>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::U16 => run::<u16>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::U24 => run::<U24>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::U32 => run::<u32>(out_samples, &out_device, out_config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::U64 => run::<u64>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::F32 => run::<f32>(out_samples, &out_device, out_config.into()),
        cpal::SampleFormat::F64 => run::<f64>(out_samples, &out_device, out_config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }.unwrap();
    
    println!("Done");
}

// fn wav_sample_format(format: cpal::SampleFormat) -> hound::SampleFormat {
//     if format.is_float() {
//         hound::SampleFormat::Float
//     } else {
//         hound::SampleFormat::Int
//     }
// }

// fn wav_spec_from_config(config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
//     hound::WavSpec {
//         channels: config.channels() as _,
//         sample_rate: config.sample_rate() as _,
//         bits_per_sample: (config.sample_format().sample_size() * 8) as _,
//         sample_format: wav_sample_format(config.sample_format()),
//     }
// }

fn convert_sample_rate(data: &Vec<f32>, input_sample_rate: u32, out_sample_rate: u32, channels: u16) -> Vec<f32> {
    samplerate::convert(
        input_sample_rate as _,
        out_sample_rate as _,
        channels as _,
        samplerate::ConverterType::SincBestQuality,
        data,
    )
    .unwrap_or_default()
}

fn convert_channels(in_samples: &Vec<f32>, in_channels: usize, out_channels: usize) -> Vec<f32> {
    if in_channels == out_channels {
        println!("Preserving channel count");
        in_samples.clone()
    } else if in_channels == 1 {
        // 1 -> 2 - duplicate the left channel
        println!("Dupe to stereo");
        let mut new_samples : Vec<f32> = Vec::new();
        for sample in in_samples.iter() {
            new_samples.push(*sample);
            new_samples.push(*sample);
        }
        new_samples
    } else if in_channels == 2 {
        // 2 -> 1 - average
        // mix down stereo to mono
        println!("Mix to mono");
        in_samples
            .chunks(2) // Iterate over pairs of samples (left, right)
            .map(|chunk| {
                let left = chunk[0];
                let right = chunk[1];
                let mono = (left + right) / 2.0; // Average the two channels
                (mono * i16::MAX as f32).round() as f32
            })
            .collect()
    } else {
        assert!(false);
        in_samples.clone()
    }
}

fn get_modulus(arg_speed: Speed) -> u32 {
    match arg_speed {
        Speed::Slow => 30,
        Speed::Normal => 15,
        Speed::Fast => 10,
        Speed::Turbo => 6,
    }
}

// the generic type is the input format -> f32
fn samples_from_wav(input_wav: &mut hound::WavReader<std::io::BufReader<std::fs::File>>, out_config: &cpal::StreamConfig) -> Vec<f32>
{
    let input_len = input_wav.len();
    println!("input_samples: {}", input_len);

    let input_frames = input_wav.duration();
    println!("input_frames: {}", input_frames);

    let input_spec = input_wav.spec();
    println!("in_spec: {:?}", input_spec);

    let input_seconds = input_frames/input_spec.sample_rate;
    println!("input_seconds: {}", input_seconds);

    // convert sample format to f32 before rate convert
    let mut input_wav_f32: Vec<f32> = Vec::new();

    match input_spec.sample_format {
        hound::SampleFormat::Float => {
            // f -> f32
            for wav_sample in input_wav.samples::<f32>() {
                let sample = wav_sample.unwrap();
                let mut sample: f32 = f32::to_sample(sample);
                sample = sample.mul_amp(0.25);
                input_wav_f32.push(sample);
            }
        },
        hound::SampleFormat::Int => {
            // i to f32
            for wav_sample in input_wav.samples::<i32>() {
                let sample = wav_sample.unwrap();
                let mut sample: f32 = i32::to_sample(sample);
                sample = sample.mul_amp(32000.0);
                input_wav_f32.push(sample);
            }
        }
    }

    let input_wav_f32_resampled = convert_sample_rate(
        &input_wav_f32, input_spec.sample_rate, out_config.sample_rate, 
        input_spec.channels
    );

    convert_channels(
        &input_wav_f32_resampled, 
        input_spec.channels as usize, 
        out_config.channels as usize
    )
}

// T is the output sample
fn run<T>(out_samples: Vec<f32>, device: &cpal::Device, out_config: cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::SizedSample + cpal::FromSample<f32>,
{
    let mut out_sample_iter = out_samples.into_iter();
    let out_sample_count = out_sample_iter.len();
    println!("Out_sample_count: {:?}", out_sample_count);

    let out_frame_count = out_sample_count / out_config.channels as usize;
    println!("out_frames {}", out_frame_count);

    let out_sample_rate = out_config.sample_rate as f32;
    println!("out_sample_rate: {}", out_sample_rate);

    let out_data_fn = move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
        let mut input_fell_behind = false;
        for frame in data.chunks_mut(out_config.channels as usize) {
            for sample in frame.iter_mut() {
                let signal_value = match out_sample_iter.next() {
                    Some(s) => s,
                    None => {
                        input_fell_behind = true;
                        0.0
                    }
                };
                *sample = T::from_sample(signal_value);
            }
        }
        if input_fell_behind {
            eprintln!("input stream fell behind");
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {err}");

    // config gets set here!
    let stream = device.build_output_stream(
        &out_config,
        out_data_fn,
        err_fn,
        None,
    )?;
    stream.play()?;

    let play_time:f32 = out_frame_count as f32/out_sample_rate;

    println!("Duration {} secs", play_time);
    std::thread::sleep(std::time::Duration::from_millis(1000 * play_time as u64));

    Ok(())
}
