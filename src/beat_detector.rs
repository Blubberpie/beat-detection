#![allow(unused)] // TODO remove

use crate::sound_processor;

use rayon::prelude::*;
use rustfft::num_complex::Complex;

use std::time::SystemTime;
use rustfft::num_traits::zero;

const BUFFER_SIZE: usize = 1024; // Size of each window
const SUB_BAND_SIZE: usize = 32; // Size of each band within each window
const ENERGY_BUFF_SIZE: usize = 43; // Size of energy history vector
const SENSIBILITY: usize = 250; // Helps determine whether or not a beat is detected

pub fn detect_beat(samples: Vec<Complex<f32>>) -> Vec<usize> {
    let mut windows = samples.chunks(BUFFER_SIZE).collect::<Vec<_>>();

    let now = SystemTime::now();
    let freq_amplitudes: Vec<Vec<f32>> =
        windows
            .par_iter()
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

    let (first_few, tail) = energies.split_at(ENERGY_BUFF_SIZE);

    // A collection of energies computed per sub-band
    let mut energy_history_buffer: Vec<Vec<f32>> = Vec::new();

    // Index of sub-bands upon which beats were detected
    let mut beat_indexes: Vec<usize> = Vec::new();

    let now = SystemTime::now();
    /// Loops through the first n=ENERGY_BUFF_SIZE windows, and collects the
    /// energy computed for each sub-band.
    for (i, wdw) in first_few.iter().enumerate() {
        for (b_i, band) in wdw.iter().enumerate() {
            if i == 0 {
                energy_history_buffer.push(vec![*band]);
            } else if let Some(b_i_history) = energy_history_buffer.get_mut(b_i) {
                b_i_history.push(*band);
            }
        }
    }

    for (i, wdw) in tail.iter().enumerate() {
        for (b_i, band) in wdw.iter().enumerate() {
            if i == 5000 && b_i == 0 { println!("{:?}", band); }
            let last_avg: f32 = energy_history_buffer.get(b_i).unwrap().iter().sum::<f32>() / ENERGY_BUFF_SIZE as f32;

            if band > &(SENSIBILITY as f32 * &last_avg) {
                beat_indexes.push((ENERGY_BUFF_SIZE + i) * (SUB_BAND_SIZE) - (SUB_BAND_SIZE - b_i) );
            }

            if let Some(b_i_history) = energy_history_buffer.get_mut(b_i) {
                b_i_history.push(*band);
                b_i_history.swap_remove(0);
            }
        }
    }
    println!("Looking for beats... \t\t\t\t\t\t{:?}", now.elapsed());

    println!("{:?}", beat_indexes.len());

    let mut beat_distance: Vec<u32> = Vec::new();

    for (i, beat_pos) in beat_indexes.iter().enumerate() {
        if i != 0 {
            let dist = (beat_pos * SUB_BAND_SIZE) - (beat_indexes.get(i-1).unwrap() * SUB_BAND_SIZE);
            beat_distance.push(dist as u32);
        }
    }

    // println!("{:?}", beat_indexes);
    // println!("{:?}", beat_distance);

    beat_indexes
}

/// Returns a vector of energies, where each index corresponds to a sub-band within the
/// given window.
///
fn compute_energies(window: &Vec<f32>) -> Vec<f32> {
    window.chunks(SUB_BAND_SIZE)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|sub_band| sub_band.iter().sum::<f32>() * SUB_BAND_SIZE as f32 / BUFFER_SIZE as f32)
        .collect::<Vec<f32>>()
}