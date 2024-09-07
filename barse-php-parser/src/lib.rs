use std::{ops::RangeInclusive, u32};

pub mod ast;

const A_TO_Z: RangeInclusive<u32> = ('a' as u32)..=('z' as u32);

trait Toast {
    type Callable: Fn(&str) -> &str;
}
