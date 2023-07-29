#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::dbg_macro)]

pub mod anchor;
pub mod elements;
pub mod style;
pub mod svgutils;

// TODO move these to prelude?
macro_rules! scalar {
    ($sc:tt) => {
        pub type Scalar = $sc;
        #[allow(unused)]
        use std::$sc::consts::PI;
    };
}

scalar!(f32);

pub type Vector2 = nalgebra::Vector2<Scalar>;

#[macro_export]
macro_rules! px {
    ($p:literal) => {
        $p as Scalar
    };
}

#[macro_export]
macro_rules! mm {
    ($p:literal) => {
        $p as Scalar * 4.0 //instead of 3.78
    };
}

#[macro_export]
macro_rules! cm {
    ($p:literal) => {
        $p as Scalar * 40.0
    };
}

#[macro_export]
macro_rules! inch {
    ($p:literal) => {
        $p as Scalar * 96.0
    };
}

#[macro_export]
macro_rules! xy {
    ($x:literal, $y:literal) => {
        Vector2::new($x as Scalar, $y as Scalar)
    };
}

#[macro_export]
macro_rules! assert_relative_eq {
    ($a:expr, $b:expr) => {
        assert!(($a - $b).norm() < 8.0 * Scalar::EPSILON);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pixels() {
        assert_eq!(px!(4), mm!(1));
        assert_eq!(cm!(10.5), mm!(105));
        assert_eq!(inch!(2), px!(192));
    }

    #[test]
    fn xy() {
        assert_eq!(xy!(1, -2), Vector2::new(1.0, -2.0));
        assert_eq!(xy!(1.5, 0), 1.5 * Vector2::x());
        assert_eq!(xy!(0, -234.5), -234.5 * Vector2::y());
        assert_eq!(xy!(0, 0), 1.5 * Vector2::zeros());
    }
}

// TODO
// - text
// - arrow
// - a vec![] like macro that implements setter and getter functions for
//   primitive types (like x, y, cx, cy, radius, etc)
