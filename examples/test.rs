use approximate::AtomicCounter;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::time::Instant;
use core::sync::atomic::Ordering;
use core::sync::atomic::AtomicU32;
use approximate::ApproximateAtomic;

fn main() {
    let count = ApproximateAtomic::<u32>::default();
    
    let time = Instant::now();
    (0..1000000000).into_par_iter().for_each(|_| {count.increment();});

    let res = Instant::now() - time;

    let c0 = count.load();

    println!("approx: {}, time: {}ms", c0, res.as_millis());

    let mut count1 = AtomicU32::new(0);
    
    let time = Instant::now();
    (0..1000000000).into_par_iter().for_each(|_| {count1.fetch_add(1, Ordering::Relaxed);});

    let res1 = Instant::now() - time;

    let c1 = *count1.get_mut();

    println!("counter: {}, time: {}ms", c1, res1.as_millis());
}