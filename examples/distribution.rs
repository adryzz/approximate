use approximate::AtomicCounter;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use approximate::ScalableCounter;
use indicatif::{ProgressBar, ProgressStyle};

fn main() {

    let len = 10000;
    
    let progress_style = ProgressStyle::default_spinner()
    .progress_chars("*-")
    .template("{spinner:.green} [{elapsed_precise}] [{bar:.green/white}] ({eta_precise})")
    .unwrap();
    let bar = ProgressBar::new(len as u64).with_style(progress_style);

    let mut counts: Vec<u32> = Vec::with_capacity(len);
    
    for _ in 0..len {
        let count = ScalableCounter::<u32>::default();

        (0..10000000).into_par_iter().for_each(|_| {count.increment();});
    
        let c0 = count.load();

        counts.push(c0);

        bar.inc(1);
    }
    let avg: u64 = counts.iter().map(|n| *n as u64).sum::<u64>() / (len as u64);

    println!("Min: {}\nMax: {}\nAvg: {}", counts.iter().min().unwrap(), counts.iter().max().unwrap(), avg);
}