#![allow(unused)]

mod beat_detector;
mod sound_processor;
mod visualizer;

use rayon::prelude::*;
use std::time::SystemTime;
use std::env;
use std::sync::Arc;
use std::fmt::Pointer;

fn main() {
    let mut reader = sound_processor::load_file("gorillaz");

    let mut raw_samples = reader.samples::<i16>(); // S = bits per sample (i16 in this case)

    let now = SystemTime::now();
    let samples = sound_processor::to_complex(raw_samples);
    println!("Converting samples to complex numbers... \t{:?}", now.elapsed());

    println!("{:?} beats per minute!", beat_detector::detect_beat(samples));

}