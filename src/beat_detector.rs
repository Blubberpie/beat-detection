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
const SAMPLES_PER_BEAT: usize = 6300; // Samples per beat assuming 400 BPM (= 44100Hz / 7 beats per second)
const SAMPLE_RATE: usize = 44100;

/// Returns the BPM of a song
pub fn detect_beat(samples: Vec<Complex<f32>>) -> f32 {
    // let total_time = samples.len() as f32 / SAMPLE_RATE as f32 / 60.0; // in minutes

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
    /// Loop through the first n=ENERGY_BUFF_SIZE windows, and collects the
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

    /// Loop through the rest of the windows, while computing the average energy
    /// per sub-band each time. This pushes a new value while removing the oldest value
    /// in the history buffer.
    ///
    /// If the current energy value of the band is larger than some constant C * Average Energy,
    /// then we have a beat.
    for (i, wdw) in tail.iter().enumerate() {
        for (b_i, band) in wdw.iter().enumerate() {
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

    let mut beat_distances: Vec<u32> = Vec::new();

    // let mut beat_dist_grouped: Vec<(u32, usize)> = Vec::new();
    let mut filtered_beat_dists: Vec<u32> = Vec::new();

    for (i, beat_pos) in beat_indexes.iter().enumerate() {
        if i != 0 {
            let dist = ((beat_pos * SUB_BAND_SIZE) - (beat_indexes.get(i-1).unwrap() * SUB_BAND_SIZE)) as u32;

            /// Compare to minimum samples per beat
            // if dist > (SAMPLES_PER_BEAT / SUB_BAND_SIZE) as u32{
            if dist > SAMPLES_PER_BEAT as u32 {
                // beat_dist_grouped.push((dist, i));
                filtered_beat_dists.push(dist);
            }

            beat_distances.push(dist);
        }
    }

    // println!("{:?}", beat_indexes);
    // println!("{:?}", beat_distances);
    // println!("{:?}", beats_grouped);

    let average = avg(&filtered_beat_dists);
    let std_dev = std_deviation(&filtered_beat_dists, average).unwrap();
    filtered_beat_dists.retain(|dist| (average.unwrap() - *dist as f32).abs() <= std_dev/2.0);

    let new_avg = avg(&filtered_beat_dists);
    SAMPLE_RATE as f32 / new_avg.unwrap() * 60.0
}

/// Returns a vector of energies, where each index corresponds to a sub-band within the
/// given window.
fn compute_energies(window: &Vec<f32>) -> Vec<f32> {
    window.chunks(SUB_BAND_SIZE)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|sub_band| sub_band.iter().sum::<f32>() * SUB_BAND_SIZE as f32 / BUFFER_SIZE as f32)
        .collect::<Vec<f32>>()
}

/// Disclaimer: The following lines of code for avg and std_deviation were copied from:
/// https://rust-lang-nursery.github.io/rust-cookbook/science/mathematics/statistics.html

fn avg(data: &Vec<u32>) -> Option<f32> {
    let sum = data.iter().sum::<u32>();
    let count = data.len();

    match count {
        positive if positive > 0 => Some(sum as f32 / count as f32),
        _ => None,
    }
}

fn std_deviation(data: &Vec<u32>, average: Option<f32>) -> Option<f32> {
    match (average, data.len()) {
        (Some(data_mean), count) if count > 0 => {
            let variance = data.iter().map(|value| {
                let diff = data_mean - (*value as f32);

                diff * diff
            }).sum::<f32>() / count as f32;

            Some(variance.sqrt())
        },
        _ => None
    }
}