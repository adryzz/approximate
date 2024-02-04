use std::{sync::atomic::*, fmt::Debug, ops::Add};

use rand::Rng;

/// This represents an atomic counter that uses probability 
/// to achieve faster operations, while still retaining a good accuracy.
/// 
/// Very useful for statistic displays in hot loops.
#[derive(Debug, Default)]
pub struct ApproximateAtomic<T> where T : Countable {
    count: T::Atomic
}

impl<T> ApproximateAtomic<T> where T : Countable {
    /// Creates a new [ApproximateAtomic], with a value of 0
    pub fn new() -> Self {
        Self { count: T::Atomic::new(<T::Atomic as Atomic>::Ticket::ZERO) }
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

pub trait Atomic : Debug + Default {
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
                        let log_count = <$primitive>::BITSM - count.leading_zeros();
            
                        if log_count >= 13 {
                            let delta = 1 << (log_count - 12);
                            let rand: $primitive = rand::thread_rng().gen();
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
                    self.count.store(<$primitive>::ZERO, std::sync::atomic::Ordering::Relaxed)
                }

                fn load(&self) -> $primitive {
                    self.count.load(Ordering::Relaxed)
                }
            }

            impl Atomic for $atomic {
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
                const BITSM: u32 = <$primitive>::BITS - 1;
            }

            impl From<$primitive> for ApproximateAtomic<$primitive> {
                fn from(value: $primitive) -> Self {
                    Self { count: <$atomic>::new(value) }
                }
            }
        )*
    };
}

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

pub trait Countable where Self : Copy + Add + Debug + Default {
    type Atomic : Atomic;
    const ZERO: Self;
    const ONE: Self;
    const BITSM: u32;
}