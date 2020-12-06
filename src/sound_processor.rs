#![allow(unused)]

use hound;
use rayon::prelude::*;

use std::fs::File;
use std::env;
use hound::{WavReader, WavSamples};
use std::io::BufReader;

use rustfft::FFTplanner;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Zero, Pow};
use std::f32::consts::PI;

pub fn load_file(song_title: &str) -> WavReader<BufReader<File>> {
    let mut path = env::current_dir().unwrap();
    path.push("src");
    path.push("sounds");
    path.push(song_title);
    path.set_extension("wav");

    let mut reader = hound::WavReader::open(path).unwrap();

    reader
}

/// Converts each sample to a Complex value
///
/// # Arguments
/// * `samples` - An iterator of wav samples
pub fn to_complex(samples: WavSamples<BufReader<File>, i16>) -> Vec<Complex<f32>> {
    samples
        .map(|x| Complex::new(x.unwrap() as f32, 0f32))
        .collect::<Vec<Complex<f32>>>()
}

/// Computes and returns the FFT of complex samples
///
/// # Arguments
/// * `num_samples` - The sample count of the input
/// * `input` - A vector of samples of complex values
pub fn get_fft(input: &mut Vec<Complex<f32>>, num_samples: usize) -> Vec<Complex<f32>> {
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(num_samples);

    let mut spectrum = vec![Complex::zero(); num_samples];
    fft.process(&mut input[..], &mut spectrum);
    spectrum
}

pub fn get_freq_amplitudes(input: &mut Vec<Complex<f32>>) -> Vec<f32> {
    get_fft(input, input.len())
        .iter()
        .map(|sample| sample.norm_sqr())
        .collect::<Vec<f32>>()
}

/// Returns the maximum frequency of a given spectrum vector
pub fn find_max_freq(spectrum: Vec<Complex<f32>>, num_samples: usize) -> f32 {
    spectrum
        .iter()
        .take(num_samples / 2)
        .map(|val| val.re)
        .max_by(|a, b| a.partial_cmp(&b).unwrap())
        .unwrap()
}

/// Returns a Hamming window computed on the input sample vector
///
/// A **Hamming window** takes care of spectral leakage when performing
/// FFT or sampling on audio.
pub fn hamming_window(samples: &Vec<Complex<f32>>) -> Vec<Complex<f32>> {
    let n_samples = samples.len();
    /// Returns the hamming weight of a given sample
    ///
    /// alpha - ( beta * cos( (2 * pi * n) / N - 1 ) )
    /// alpha = 0.54
    /// beta = 1 - alpha = 0.46
    /// N = window size
    fn hamming(index: usize, sample: Complex<f32>, num_samples: usize) -> Complex<f32> {
        let temp: f32 = (2.0 * PI * index as f32) / (num_samples as f32 - 1.0);
        let ham = 0.54 - (0.46 * temp.cos());
        sample * ham
    }

    samples
        .iter()
        .cloned()
        .enumerate()
        .map(|(i, sample)| hamming(i, sample, n_samples))
        .collect()
}
