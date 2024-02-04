use approximate::AtomicCounter;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::time::Instant;
use atomic::*;
use approximate::ScalableCounter;
use rand::*;

/// This example uses thread RNG, but with a no_std counter.
fn main() {
    let count = ScalableCounter::<u128>::with_rng(thread_rng);
    
    let time = Instant::now();
    (0..1000000000).into_par_iter().for_each(|_| {count.increment();});

    let res = Instant::now() - time;

    let c0 = count.load();

    println!("approx: {}, time: {}ms", c0, res.as_millis());

    let mut count1 = Atomic::<u128>::new(0);
    
    let time = Instant::now();
    (0..1000000000).into_par_iter().for_each(|_| {count1.fetch_add(1, Ordering::Relaxed);});

    let res1 = Instant::now() - time;

    let c1 = *count1.get_mut();

    println!("counter: {}, time: {}ms", c1, res1.as_millis());
}

fn thread_rng() -> u128 {
    rand::thread_rng().gen()
}