mod sound_processor;
mod visualizer;

use rustfft;
use gnuplot::{Figure, Caption, Color};
use std::env;

use std::sync::Arc;

fn main() {
    let mut reader = sound_processor::load_file();
    let samp = reader.samples::<i16>(); // S = bits_per_sample (i16 in this case)
    let data = samp.flatten().collect::<Vec<_>>();
    let mut ind = Vec::new();

    for i in 0..data.len() {
        ind.push(i);
    }

    let mut fg = Figure::new();

    fg.axes2d().lines(&ind, &ind, &[Caption("A line"), Color("black")]);
    fg.show();
    // println!("{:?}", data);

    // let mut path = env::current_dir().unwrap();
    // path.push("src");
    // path.push("png");
    // path.push("test");
    // path.set_extension("png");
    // println!("{:?}", fg.save_to_png(path, 700, 600));

    // let mut input: Vec<Complex<f32>> = vec![Complex::zero(); 1234];
    // let mut output: Vec<Complex<f32>> = vec![Complex::zero(); 1234];
    //
    // let mut planner = FFTplanner::new(false);
    // let fft = planner.plan_fft(1234);
    // fft.process(&mut input, &mut output);
    //
    // let fft_clone = Arc::clone(&fft);
    //
    // println!("{:?}", input)
}
