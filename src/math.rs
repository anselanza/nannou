extern crate cgmath;
pub use self::cgmath::*;

use self::cgmath::num_traits::{NumCast, One};
use std::ops::Add;

pub fn pt1<S>(x: S) -> Point1<S> {
    Point1 { x }
}

pub fn pt2<S>(x: S, y: S) -> Point2<S> {
    Point2 { x, y }
}

pub fn pt3<S>(x: S, y: S, z: S) -> Point3<S> {
    Point3 { x, y, z }
}

/// Map a value from a given range to a new given range.
pub fn map_range<X, Y>(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y
where
    X: NumCast,
    Y: NumCast,
{
    let val_f: f64 = NumCast::from(val).unwrap();
    let in_min_f: f64 = NumCast::from(in_min).unwrap();
    let in_max_f: f64 = NumCast::from(in_max).unwrap();
    let out_min_f: f64 = NumCast::from(out_min).unwrap();
    let out_max_f: f64 = NumCast::from(out_max).unwrap();
    NumCast::from(
        (val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f
    ).unwrap()
}

/// The max between two partially ordered values.
pub fn partial_max<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a >= b { a } else { b }
}

/// The min between two partially ordered values.
pub fn partial_min<T>(a: T, b: T) -> T
where
    T: PartialOrd,
{
    if a <= b { a } else { b }
}

/// Clamp a value between some range.
pub fn clamp<T>(n: T, start: T, end: T) -> T
where
    T: PartialOrd,
{
    if start <= end {
        if n < start { start } else if n > end { end } else { n }
    } else {
        if n < end { end } else if n > start { start } else { n }
    }
}

pub fn two<S>() -> S
where
    S: Add<Output=S> + One,
{
    S::one() + S::one()
}
