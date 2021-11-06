use core::{iter::Copied, marker::PhantomData, ptr::NonNull};

use alloc::{boxed::Box, string::String};

use crate::{CharDecoder, ZBytesCreationError, ZBytesRefIter, ZStr};

/// Owns a non-null pointer to zero-termianted bytes.
///
/// Like with a `String`, the bytes **must** be utf-8 encoded.
///
/// Because this is a "thin" pointer it's suitable for direct use with FFI.
#[repr(transparent)]
#[cfg_attr(docs_rs, doc(cfg(feature = "alloc")))]
pub struct ZString {
  pub(crate) nn: NonNull<u8>,
}
impl Drop for ZString {
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
    unsafe {
      Box::from_raw(core::str::from_utf8_unchecked_mut(
        core::slice::from_raw_parts_mut(self.nn.as_ptr(), len),
      ))
    };
  }
}
impl core::fmt::Debug for ZString {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    core::fmt::Debug::fmt(&self.get_ref(), f)
  }
}
impl core::fmt::Display for ZString {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    core::fmt::Display::fmt(&self.get_ref(), f)
  }
}
impl core::fmt::Pointer for ZString {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    core::fmt::Pointer::fmt(&self.get_ref(), f)
  }
}
impl<'a> From<ZStr<'a>> for ZString {
  #[inline]
  #[must_use]
  fn from(zs: ZStr<'a>) -> Self {
    let b = String::from(zs.as_str_including_null()).into_boxed_str();
    unsafe { Self::from_boxed_str_unchecked(b) }
  }
}
impl TryFrom<Box<str>> for ZString {
  type Error = ZBytesCreationError;

  #[inline]
  #[must_use]
  fn try_from(b: Box<str>) -> Result<Self, Self::Error> {
    match b.as_bytes().split_last() {
      None => Err(ZBytesCreationError::NullTerminatorMissing),
      Some((terminator, data)) => {
        if terminator != &0 {
          Err(ZBytesCreationError::NullTerminatorMissing)
        } else if data.iter().any(|u| u == &0) {
          Err(ZBytesCreationError::InteriorNull)
        } else {
          Ok(unsafe { Self::from_boxed_str_unchecked(b) })
        }
      }
    }
  }
}
impl ZString {
  /// Get the borrowed form of the data.
  #[inline]
  #[must_use]
  pub const fn get_ref<'a>(&'a self) -> ZStr<'a> {
    ZStr { nn: self.nn, marker: PhantomData }
  }

  /// Gets an iterator over the bytes.
  ///
  /// The iterator does **not** return the final null byte.
  #[inline]
  #[must_use]
  pub const fn iter_bytes<'a>(&'a self) -> ZBytesRefIter<'a> {
    ZBytesRefIter { nn: self.nn, marker: PhantomData }
  }

  /// Gets an iterator for the characters of the string data.
  #[inline]
  #[must_use]
  pub fn chars<'a>(&'a self) -> CharDecoder<Copied<ZBytesRefIter<'a>>> {
    CharDecoder::from(self.iter_bytes().copied())
  }

  /// Converts a boxed `str` into a `ZString`
  ///
  /// ## Safety
  /// * The final byte in the str must be 0.
  /// * All other bytes in the str must be non-zero.
  #[inline]
  #[must_use]
  pub unsafe fn from_boxed_str_unchecked(b: Box<str>) -> Self {
    Self { nn: NonNull::new_unchecked(Box::leak(b).as_mut_ptr()) }
  }
}
