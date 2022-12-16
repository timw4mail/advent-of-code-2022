pub mod grid;
pub mod enums;

pub use grid::*;

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
