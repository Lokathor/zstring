#![cfg_attr(not(test), no_std)]
#![warn(missing_docs)]

//! A crate to make zero-termiated FFI data easier to work with.

macro_rules! impl_zbytes_fmt {
  ($imp_target:ty: $($t:ident),*) => {
    $(
      impl<'a> core::fmt::$t for $imp_target {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          write!(f, "[")?;
          for (i, b) in self.iter().enumerate() {
            if i != 0 {
              write!(f, ", ")?;
            }
            core::fmt::$t::fmt(b, f)?;
          }
          write!(f, "]")?;
          Ok(())
        }
      }
    )*
  }
}

/// An error when you tried to make a z-bytes variant.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ZBytesCreationError {
  /// An interior null was detected.
  InteriorNull,
  /// There was no 0 value at the end.
  NullTerminatorMissing,
}

mod zbytes_ref;
pub use zbytes_ref::*;

mod zbytes_mut;
pub use zbytes_mut::*;
