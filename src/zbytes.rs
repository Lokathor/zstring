use core::{
  marker::PhantomData,
  ptr::{slice_from_raw_parts_mut, NonNull},
};

use alloc::boxed::Box;

use crate::{ZBytesCreationError, ZBytesRef, ZBytesRefIter};

/// Owns a non-null pointer to some zero-terminated bytes.
///
/// The bytes have no enforced encoding.
///
/// Because this is a "thin" pointer it's suitable for direct use with FFI.
///
/// Because the size of the allocation isn't stored, the amount of data stored
/// in this allocation can't be changed, including that bytes within the
/// sequence cannot be set to 0.
#[repr(transparent)]
#[cfg_attr(docs_rs, doc(cfg(feature = "alloc")))]
pub struct ZBytes {
  pub(crate) nn: NonNull<u8>,
}
impl Drop for ZBytes {
  fn drop(&mut self) {
    let len = {
      let mut x = 1;
      let mut p = self.nn.as_ptr();
      while unsafe { *p } != 0 {
        x += 1;
        p = unsafe { p.add(1) };
      }
      x
    };
    unsafe { Box::from_raw(slice_from_raw_parts_mut(self.nn.as_ptr(), len)) };
  }
}
impl_zbytes_fmt!(
  ZBytes: Binary,
  Debug,
  Display,
  LowerExp,
  LowerHex,
  Octal,
  UpperExp,
  UpperHex
);
impl core::fmt::Pointer for ZBytes {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.write_str("ZBytes(")?;
    core::fmt::Pointer::fmt(&self.nn.as_ptr(), f)?;
    f.write_str(")")?;
    Ok(())
  }
}
impl TryFrom<Box<[u8]>> for ZBytes {
  type Error = ZBytesCreationError;

  #[inline]
  #[must_use]
  fn try_from(b: Box<[u8]>) -> Result<Self, Self::Error> {
    match b.split_last() {
      None => Err(ZBytesCreationError::NullTerminatorMissing),
      Some((terminator, data)) => {
        if terminator != &0 {
          Err(ZBytesCreationError::NullTerminatorMissing)
        } else if data.iter().any(|u| u == &0) {
          Err(ZBytesCreationError::InteriorNull)
        } else {
          Ok(unsafe { Self::from_boxed_slice_unchecked(b) })
        }
      }
    }
  }
}
impl ZBytes {
  /// Gets an iterator over the bytes.
  ///
  /// The iterator does **not** return the final null byte.
  #[inline]
  #[must_use]
  pub const fn iter<'a>(&'a self) -> ZBytesRefIter<'a> {
    ZBytesRefIter { nn: self.nn, marker: PhantomData }
  }

  /// Get the borrowed form of the data.
  #[inline]
  #[must_use]
  pub const fn get_ref<'a>(&'a self) -> ZBytesRef<'a> {
    ZBytesRef { nn: self.nn, marker: PhantomData }
  }

  /// Converts a boxed slice of bytes into a `ZBytes`
  ///
  /// ## Safety
  /// * The final byte in the slice must be 0.
  /// * All other bytes in the slice must be non-zero.
  #[inline]
  #[must_use]
  pub unsafe fn from_boxed_slice_unchecked(b: Box<[u8]>) -> Self {
    Self { nn: NonNull::new_unchecked(Box::leak(b).as_mut_ptr()) }
  }
}
