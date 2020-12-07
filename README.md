# beat-detection

This is an (attempted) implementation of the Frequency Selected Sound Energy Algorithm found here: 
http://archive.gamedev.net/archive/reference/programming/features/beatdetection/ 

## Disclaimers

First of all, the final returned BPM does not work properly. Although I believe I can say that the onset detector itself can properly detect beats, but way too frequently. 
These "hot areas" of detected beats signify the general location of the actual beat, but I couldn't figure out how to locate it and find the BPM.

If this project was one that showed the locations of each beat using some plotting library, then it would have been much more successful. But since I made
it to be a BPM detector, it was a failure - at least for the BPM part.

## How it works

I divide the entire audio into windows of size 1024 samples, and divide each window into sub-bands of 32 samples. A Hamming window is applied on each sub-band, before
the FFT is taken. After computing the frequency amplitudes (squared norm) of each sub-band, the energy is calculated per sub-band. The energies of the sub-bands are
accumulated 43 at a time, with each new sub-band compared to the average of the previous 43. If this new sub-band's energy is greater than some constant C times the
average, then we have a beat. Here, C is set to be 250. This generally works around modern music. (Testing on chip-tune music does not yield proper results)
The indexes of each detected beat is recorded and saved into a new array. That is the general idea for the first portion.

## The BPM detection

As for detecting the BPM, I used the saved indices of the beats and calculated the distances (number of samples) between each beat. With this number, I found the
average distance as well as the standard deviation, hoping that I can find some common duration with which I could compute the BPM. However, there must have been
some miscalculation or a big hole in the algorithm since the results are never accurate.

## Parallelism

I used parallelism wherever required, and it made very significant differences in performance. With computations of lists of smaller sizes though, Rust's `rayon::par_iter()`
performs much worse than sequential computations. When iterating through windows and sub-bands, parallelism excelled. Currently, an input `wav` file of 4 and a half
minutes yields the final result within 30 seconds. 

Mutability was avoided as much as possible, although in many cases it was necessary.

## Conclusion

Overall, I learned quite a lot about Rust. Given a little more time, I probably could have properly calculated the BPM using the beat indexes I generated. Ultimatly,
this was a semi-failed project and I think I could have done better.
