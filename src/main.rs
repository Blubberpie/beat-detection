#![allow(unused)]

mod sound_processor;
mod visualizer;

use rayon::prelude::*;
use std::time::SystemTime;
use std::env;
use std::sync::Arc;
use std::fmt::Pointer;

const BUFFER_SIZE: usize = 1024;
const SUB_BAND_SIZE: usize = 32;
const ENERGY_BUFF_SIZE: usize = 43;

fn main() {
    let now = SystemTime::now();
    let mut reader = sound_processor::load_file();
    let num_samples_total = reader.len() as usize;
    println!("{:?}", now.elapsed());

    let mut raw_samples = reader.samples::<i16>();

    let now = SystemTime::now();
    // S = bits_per_sample (i16 in this case)
    let samples = sound_processor::to_complex(raw_samples);
    println!("{:?}", now.elapsed());
    // let start = 100000;

    // let now = SystemTime::now();
    // let mut hamming_samples = sound_processor::hamming_window(&samples[start..start + BUFFER_SIZE].to_vec(), BUFFER_SIZE);
    // println!("{:?}", now.elapsed());
    // // println!("{:?}", hamming_samples);
    // let spectrum_slice = sound_processor::get_fft(&mut hamming_samples, BUFFER_SIZE);
    // println!("{:?}", spectrum_slice);
    //
    // let now = SystemTime::now();
    // println!("{:?}", sound_processor::find_max_freq(spectrum_slice, BUFFER_SIZE));
    // println!("{:?}", now.elapsed());

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