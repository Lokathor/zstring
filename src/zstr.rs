use core::{iter::Copied, marker::PhantomData, ptr::NonNull};

use crate::{CharDecoder, ZBytesRef, ZBytesRefIter};

/// Borrows a non-null **const** pointer to zero-terminated bytes.
///
/// Like with a `str`, the bytes **must** be utf-8 encoded.
///
/// Because this is a "thin" pointer it's suitable for direct use with FFI.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct ZStr<'a> {
  pub(crate) nn: NonNull<u8>,
  pub(crate) marker: PhantomData<&'a [u8]>,
}
impl<'a, 'b> PartialEq<ZStr<'b>> for ZStr<'a> {
  #[inline]
  #[must_use]
  fn eq(&self, other: &ZStr<'b>) -> bool {
    self.as_zbytes_ref() == other.as_zbytes_ref()
  }
}
impl<'a> Eq for ZStr<'a> {}
impl<'a> PartialEq<&str> for ZStr<'a> {
  #[inline]
  #[must_use]
  fn eq(&self, other: &&str) -> bool {
    self.as_zbytes_ref() == other.as_bytes()
  }
}
impl<'a> PartialEq<ZStr<'a>> for &str {
  #[inline]
  #[must_use]
  fn eq(&self, other: &ZStr<'a>) -> bool {
    other.as_zbytes_ref() == self.as_bytes()
  }
}
impl<'a> core::fmt::Debug for ZStr<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mut buffer = [0; 4_usize];
    f.write_str("\"")?;
    for ch in self.chars() {
      f.write_str(ch.encode_utf8(&mut buffer))?;
    }
    f.write_str("\"")?;
    Ok(())
  }
}
impl<'a> core::fmt::Display for ZStr<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    let mut buffer = [0; 4_usize];
    Ok(for ch in self.chars() {
      f.write_str(ch.encode_utf8(&mut buffer))?;
    })
  }
}
impl<'a> core::fmt::Pointer for ZStr<'a> {
  fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
    f.write_str("ZStr(")?;
    core::fmt::Pointer::fmt(&self.nn.as_ptr(), f)?;
    f.write_str(")")?;
    Ok(())
  }
}
impl<'a> ZStr<'a> {
  /// Turns a `NonNull` into a `ZStr`
  ///
  /// ## Safety
  /// * The NonNull must point to a series of utf-8 encoded bytes that is
  ///   null-terminated.
  #[inline]
  #[must_use]
  pub const unsafe fn from_non_null_unchecked(nn: NonNull<u8>) -> Self {
    Self { nn, marker: PhantomData }
  }

  /// Gets an iterator over the bytes.
  ///
  /// The iterator does **not** return the final null byte.
  #[inline]
  #[must_use]
  pub const fn iter_bytes(&self) -> ZBytesRefIter<'a> {
    ZBytesRefIter { nn: self.nn, marker: PhantomData }
  }

  /// Gets an iterator for the characters of the string data.
  #[inline]
  #[must_use]
  pub fn chars(&self) -> CharDecoder<Copied<ZBytesRefIter<'a>>> {
    CharDecoder::from(self.iter_bytes().copied())
  }

  /// Looks at the underlying data as bytes rather than as a str.
  #[inline]
  #[must_use]
  pub const fn as_zbytes_ref(&self) -> ZBytesRef<'a> {
    ZBytesRef { nn: self.nn, marker: PhantomData }
  }

  /// Gets the full str this points to, **including the null byte.**
  ///
  /// **Caution:** This takes linear time to compute the slice length!
  #[inline]
  #[must_use]
  pub fn as_str_including_null(&self) -> &'a str {
    let mut count = 1;
    let mut p = self.nn.as_ptr();
    while unsafe { *p } != 0 {
      count += 1;
      p = unsafe { p.add(1) };
    }
    unsafe {
      core::str::from_utf8_unchecked(core::slice::from_raw_parts(
        self.nn.as_ptr(),
        count,
      ))
    }
  }

  /// Gets the underlying pointer
  #[inline]
  #[must_use]
  pub const fn as_ptr(&self) -> *const u8 {
    self.nn.as_ptr()
  }
}
