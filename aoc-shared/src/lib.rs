pub mod grid;
pub mod enums;

pub use grid::*;
pub use enums::*;

#[derive(Debug, Default, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Location {
    pub x: isize,
    pub y: isize,
}

impl Location {
    pub fn new(x: isize, y: isize) -> Self {
        Location { x, y }
    }

    pub fn get_distance(self, other: Self) -> f64 {
        let squares = (other.x - self.x).pow(2) + (other.y - self.y).pow(2);

        (squares as f64).sqrt()
    }
}

#[macro_export]
macro_rules! deref {
    ($($struct: ty, $target: ty),* ) => {
        $(
            impl ::core::ops::Deref for $struct {
                type Target = $target;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl ::core::ops::DerefMut for $struct {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        )*
    }
}
