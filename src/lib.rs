pub mod anchor;
pub mod nodes;
pub mod style;
pub mod transform;

macro_rules! scalar {
    ($sc:tt) => {
        type Scalar = $sc;
        #[allow(unused)]
        use std::$sc::consts::PI;
    }
}

scalar!(f32);

type Vector2 = nalgebra::Vector2<Scalar>;



#[macro_export]
macro_rules! px {
    ($p:literal) => {
        $p as Scalar
    }
}

#[macro_export]
macro_rules! mm {
    ($p:literal) => {
        $p as Scalar * 4.0  //instead of 3.78
    }
}

#[macro_export]
macro_rules! cm {
    ($p:literal) => {
        $p as Scalar * 40.0
    }
}

#[macro_export]
macro_rules! inch {
    ($p:literal) => {
        $p as Scalar * 96.0
    }
}

#[test]
fn pixels() {
    assert_eq!(px!(4), mm!(1));
    assert_eq!(cm!(10.5), mm!(105));
    assert_eq!(inch!(2), px!(192));
}

// TODO
// - text
// - line
// - arrow
// - marker
// - path
