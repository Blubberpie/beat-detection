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
    let mut reader = sound_processor::load_file("eagles");
    let num_samples_total = reader.len() as usize;

    let mut raw_samples = reader.samples::<i16>(); // S = bits per sample (i16 in this case)

    let now = SystemTime::now();
    let samples = sound_processor::to_complex(raw_samples);
    println!("Converting samples to complex numbers... \t{:?}", now.elapsed());
    // let start = 100000;

    beat_detector::detect_beat(samples);

}

// Curried function to print run time of implementations
fn time<F, T>(func: F) -> impl Fn(T) -> T
    where F: Fn(T) -> T {
    move |input| {
        let now = SystemTime::now();
        let ret = func(input);
        println!("{:?}", now.elapsed());
        ret
    }
}