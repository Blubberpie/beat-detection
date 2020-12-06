#![allow(unused)]

use crate::sound_processor;

use rayon::prelude::*;
use rustfft::num_complex::Complex;

use std::time::SystemTime;
use rustfft::num_traits::zero;

const BUFFER_SIZE: usize = 1024; // Size of each window
const SUB_BAND_SIZE: usize = 32; //
const ENERGY_BUFF_SIZE: usize = 43;
const SENSIBILITY: usize = 250; // Helps determine whether or not a beat is detected

pub fn detect_beat(samples: Vec<Complex<f32>>) {
    let mut windows = samples.chunks(BUFFER_SIZE);

    let now = SystemTime::now();
    let freq_amplitudes: Vec<Vec<f32>> =
        windows
            .par_bridge()
            .map(|chunk|
                sound_processor::get_freq_amplitudes(
                    &mut sound_processor::hamming_window(&mut chunk.to_vec())
                ))
            .collect();
    println!("Calculating frequency amplitudes... \t\t{:?}", now.elapsed());

    let now = SystemTime::now();
    let energies: Vec<Vec<f32>> = freq_amplitudes
        .par_iter()
        .map(|window| compute_energies(window))
        .collect();
    println!("Calculating energies... \t\t\t\t\t{:?}", now.elapsed());


}

/// Returns a vector of energies, where each index corresponds to a sub-band within the
/// given window.
///
fn compute_energies(window: &Vec<f32>) -> Vec<f32> {
    window.chunks(SUB_BAND_SIZE)
        .par_bridge()
        .map(|sub_band| sub_band.iter().sum::<f32>() * SUB_BAND_SIZE as f32 / BUFFER_SIZE as f32)
        .collect::<Vec<f32>>()
}