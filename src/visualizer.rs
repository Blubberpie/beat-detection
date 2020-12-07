#![allow(unused)]

use gnuplot::{Figure, Caption, Color};

/// Not implemented!
pub fn show_plot(data: Vec<i16>) {

    let mut fg = Figure::new();

    let mut ind = Vec::new();

    for i in 0..data.len() {
        ind.push(i);
    }

    fg.axes2d().lines(&ind, &ind, &[Caption("A line"), Color("black")]);
    fg.show();

    // let mut path = env::current_dir().unwrap();
    // path.push("src");
    // path.push("png");
    // path.push("test");
    // path.set_extension("png");
    // println!("{:?}", fg.save_to_png(path, 700, 600));

}