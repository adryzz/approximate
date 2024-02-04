#![cfg_attr(feature = "nostd", no_std)]


#[cfg(feature = "std")]
use std::sync::atomic::*;

use core::{fmt::Debug, ops::Add};

#[cfg(all(feature = "rand", feature = "std"))]
use rand::Rng;

#[cfg(feature = "nostd")]
use atomic::*;

/// This represents an atomic counter that uses probability 
/// to achieve faster operations, while still retaining a good accuracy.
/// 
/// Very useful for statistic displays in hot loops.
#[derive(Debug)]
pub struct ApproximateAtomic<T> where T : Countable {
    count: T::Atomic,
    rng: fn() -> T
}

impl<T> ApproximateAtomic<T> where T : Countable {
    pub fn with_rng(rng: fn() -> T) -> Self {
        Self { count: T::Atomic::new(<T::Atomic as AtomicVal>::Ticket::ZERO), rng }
    }
}

#[cfg(all(feature = "rand", feature = "std"))]
impl<T> Default for ApproximateAtomic<T> where T : Countable {
    fn default() -> Self {
        Self { count: Default::default(), rng: T::thread_rng }
    }
}

/// This trait represents all the possible operations on an [ApproximateAtomic]
pub trait AtomicCounter {
    /// The value held
    type Ticket : Countable;
    /// Increments the counter by one.
    fn increment(&self) -> Self::Ticket;
    /// Resets the counter to zero.
    fn reset(&self);
    /// Retrieves the current counter value.
    fn load(&self) -> Self::Ticket;
}

pub trait AtomicVal : Debug + Default {
    type Ticket : Countable;
    fn fetch_add(&self, value: Self::Ticket, order: Ordering) -> Self::Ticket;
    fn load(&self, order: Ordering) -> Self::Ticket;
    fn store(&self, value: Self::Ticket, order: Ordering);
    fn new(value: Self::Ticket) -> Self;
}

macro_rules! counter {
    ($($atomic:ty : $primitive:ty;)*) => {
        $(
            impl AtomicCounter for ApproximateAtomic<$primitive> {
                type Ticket = $primitive;
                fn increment(&self) -> Self::Ticket {
                    let count = self.count.load(Ordering::Relaxed);

                    let delta = if count > 0 {
                        let log_count = count.ilog2();
            
                        if log_count >= 13 {
                            let delta = 1 << (log_count - 12);
                            let rand: $primitive = (self.rng)();
                            let update: bool = (rand & (delta - 1)) == 0;
            
                            if !update {
                                return count;
                            }
            
                            delta
                        } else {<$primitive>::ONE}
                    } else {<$primitive>::ONE};
            
                    self.count.fetch_add(delta, Ordering::Relaxed)
                }

                fn reset(&self) {
                    self.count.store(<$primitive>::ZERO, Ordering::Relaxed)
                }

                fn load(&self) -> $primitive {
                    self.count.load(Ordering::Relaxed)
                }
            }

            impl AtomicVal for $atomic {
                type Ticket = $primitive;
                fn fetch_add(&self, value: Self::Ticket, order: Ordering) -> Self::Ticket {
                    self.fetch_add(value, order)
                }
                fn load(&self, order: Ordering) -> Self::Ticket {
                    self.load(order)
                }
                fn store(&self, value: Self::Ticket, order: Ordering) {
                    self.store(value, order)
                }
                fn new(value: Self::Ticket) -> Self {
                    <$atomic>::new(value)
                }
            }

            impl Countable for $primitive {
                type Atomic = $atomic;
                const ZERO: $primitive = 0;
                const ONE: $primitive = 1;

                #[cfg(all(feature = "rand", feature = "std"))]
                fn thread_rng() -> $primitive {
                    rand::thread_rng().gen()
                }
            }

            #[cfg(all(feature = "rand", feature = "std"))]
            impl From<$primitive> for ApproximateAtomic<$primitive> {
                fn from(value: $primitive) -> Self {
                    Self { count: <$atomic>::new(value), rng: <$primitive>::thread_rng }
                }
            }
        )*
    };
}

pub trait Countable where Self : Copy + Add + Debug + Default {
    type Atomic : AtomicVal;
    const ZERO: Self;
    const ONE: Self;
    #[cfg(all(feature = "rand", feature = "std"))]
    fn thread_rng() -> Self;
}

#[cfg(feature = "std")]
counter! {
    AtomicU8: u8;
    AtomicI8: i8;
    AtomicU16: u16;
    AtomicI16: i16;
    AtomicU32: u32;
    AtomicI32: i32;
    AtomicU64: u64;
    AtomicI64: i64;
    AtomicUsize: usize;
    AtomicIsize: isize;
}

#[cfg(feature = "nostd")]
counter! {
    Atomic<u8>: u8;
    Atomic<i8>: i8;
    Atomic<u16>: u16;
    Atomic<i16>: i16;
    Atomic<u32>: u32;
    Atomic<i32>: i32;
    Atomic<u64>: u64;
    Atomic<i64>: i64;
    Atomic<usize>: usize;
    Atomic<isize>: isize;
    Atomic<u128>: u128;
    Atomic<i128>: i128;
}

#[cfg(all(feature = "std", feature = "nostd"))]
compile_error!("feature \"std\" and feature \"nostd\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), not(feature = "nostd")))]
compile_error!("you need to enable at least one of \"std\" or \"nostd\"");