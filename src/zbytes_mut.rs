use crate::ZBytesRef;

use super::ZBytesCreationError;
use core::{marker::PhantomData, ptr::NonNull};

/// Borrows a non-null **mutable** pointer to zero-termianted bytes.
///
/// The bytes have no enforced encoding.
///
/// Because this is a "thin" pointer it's suitable for direct use with FFI.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ZBytesMut<'a> {
  pub(crate) nn: NonNull<u8>,
  pub(crate) marker: PhantomData<&'a mut [u8]>,
}
impl_zbytes_fmt!(
  ZBytesMut<'a>: Binary, Debug, Display, LowerExp, LowerHex, Octal, UpperExp, UpperHex
);
impl<'a> core::ops::Deref for ZBytesMut<'a> {
  type Target = ZBytesRef<'a>;
  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { core::mem::transmute(self) }
  }
}
impl<'a> core::fmt::Pointer for ZBytesMut<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    write!(f, "ZBytesMut({:p})", self.nn.as_ptr())
  }
}
impl<'a> TryFrom<&'a mut [u8]> for ZBytesMut<'a> {
  type Error = ZBytesCreationError;

  #[inline]
  fn try_from(value: &'a mut [u8]) -> Result<Self, Self::Error> {
    match value.split_last() {
      None => Err(ZBytesCreationError::NullTerminatorMissing),
      Some((terminator, data)) => {
        if terminator != &0 {
          Err(ZBytesCreationError::NullTerminatorMissing)
        } else if data.iter().any(|b| b == &0) {
          Err(ZBytesCreationError::InteriorNull)
        } else {
          Ok(Self {
            nn: unsafe { NonNull::new_unchecked(value.as_ptr() as _) },
            marker: PhantomData,
          })
        }
      }
    }
  }
}
impl<'a, const N: usize> TryFrom<&'a mut [u8; N]> for ZBytesMut<'a> {
  type Error = ZBytesCreationError;

  #[inline]
  fn try_from(value: &'a mut [u8; N]) -> Result<Self, Self::Error> {
    Self::try_from(value.as_mut())
  }
}

impl<'a> ZBytesMut<'a> {
  /// Turns a `NonNull` into a `ZBytesRef`
  ///
  /// ## Safety
  /// * The NonNull must point to a series of bytes that is null-terminated.
  #[inline]
  #[must_use]
  pub unsafe fn from_non_null_unchecked(nn: NonNull<u8>) -> Self {
    Self { nn, marker: PhantomData }
  }

  /// Gets an iterator over the bytes.
  #[inline]
  #[must_use]
  pub fn iter_mut<'b>(&'b mut self) -> ZBytesMutIter<'a, 'b> {
    ZBytesMutIter { nn: self.nn, marker: PhantomData, marker2: PhantomData }
  }
}

/// Iterator over a [ZBytesMut]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ZBytesMutIter<'a, 'b> {
  nn: NonNull<u8>,
  marker: PhantomData<&'a mut [u8]>,
  marker2: PhantomData<&'b mut ZBytesMut<'a>>,
}
impl<'a, 'b> Iterator for ZBytesMutIter<'a, 'b> {
  type Item = &'a mut u8;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    match unsafe { self.nn.as_mut() } {
      0 => None,
      other => {
        self.nn = unsafe { NonNull::new_unchecked(self.nn.as_ptr().add(1)) };
        Some(other)
      }
    }
  }
}
