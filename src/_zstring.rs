use core::{marker::PhantomData, ptr::NonNull};

use alloc::{boxed::Box, string::String};

use crate::{ZStr, ZStringError};

/// Owning and non-null pointer to zero-terminated utf-8 data.
///
/// Because this is a thin pointer it's suitable for direct FFI usage.
///
/// ## Safety
/// * This is `repr(transparent)` over a [`NonNull<u8>`].
/// * The wrapped pointer points at a sequence of valid-to-read non-zero byte
///   values followed by at least one zero byte.
/// * The `ZString` owns the data, and will free it on drop.
#[repr(transparent)]
#[cfg_attr(docs_rs, doc(cfg(feature = "alloc")))]
pub struct ZString {
  pub(crate) nn: NonNull<u8>,
}
impl Drop for ZString {
  #[inline]
  fn drop(&mut self) {
    let len = 1 + self.bytes().count();
    let slice_ptr: *mut [u8] =
      core::ptr::slice_from_raw_parts_mut(self.nn.as_ptr(), len);
    drop(unsafe { Box::from_raw(slice_ptr) })
  }
}
impl ZString {
  /// Converts a [`Box<str>`] into a [`ZString`] without any additional
  /// checking.
  ///
  /// ## Safety
  /// * The data **must** have *exactly* one null byte at the end.
  /// * The data **must not** contain interior null bytes.
  ///
  /// Breaking either of the above rules will cause the wrong amount to be freed
  /// when the `ZString` drops.
  #[inline]
  #[must_use]
  pub unsafe fn new_unchecked(b: Box<str>) -> Self {
    let p: *mut u8 = Box::leak(b).as_mut_ptr();
    let nn: NonNull<u8> = unsafe { NonNull::new_unchecked(p) };
    Self { nn }
  }

  /// Borrows this `ZString` as a `ZStr`.
  #[inline]
  #[must_use]
  pub const fn as_zstr(&self) -> ZStr<'_> {
    ZStr { nn: self.nn, life: PhantomData }
  }

  /// An iterator over the bytes of this `ZStr`.
  ///
  /// * This iterator *excludes* the terminating 0 byte.
  #[inline]
  pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
    self.as_zstr().bytes()
  }

  /// An iterator over the decoded `char` values of this `ZStr`.
  #[inline]
  pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
    self.as_zstr().chars()
  }
}
impl FromIterator<char> for ZString {
  #[inline]
  fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
    let iter = iter.into_iter();
    let no_nulls = iter.map(|ch| {
      assert_ne!(ch, '\0');
      ch
    });
    let null_on_the_end = no_nulls.chain(['\0']);
    let s = String::from_iter(null_on_the_end);
    // Safety: We've ensures that there's no nulls within the source iteration,
    // and that we've added a single null to the end of the iteration.
    unsafe { ZString::new_unchecked(s.into_boxed_str()) }
  }
}
impl TryFrom<&str> for ZString {
  type Error = ZStringError;
  /// Trims any trailing nulls and then makes a [`ZString`] from what's left.
  ///
  /// ```
  /// # use zstring::*;
  /// let zstring1 = ZString::try_from("abc").unwrap();
  /// assert!("abc".chars().eq(zstring1.chars()));
  ///
  /// let zstring2 = ZString::try_from("foo\0\0\0\0").unwrap();
  /// assert!("foo".chars().eq(zstring2.chars()));
  /// ```
  ///
  /// ## Failure
  /// * If there are any interior nulls.
  ///
  /// ```
  /// # use zstring::*;
  /// assert!(ZString::try_from("ab\0c").is_err());
  /// ```
  #[inline]
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let trimmed = value.trim_end_matches('\0');
    if trimmed.contains('\0') {
      Err(ZStringError::InteriorNulls)
    } else {
      Ok(trimmed.chars().collect())
    }
  }
}
impl core::fmt::Display for ZString {
  /// Display formats the string (without outer `'"'`).
  ///
  /// ```rust
  /// # use zstring::*;
  /// let zstring = ZString::try_from("foo").unwrap();
  /// let s = format!("{zstring}");
  /// assert_eq!("foo", s);
  /// ```
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Display::fmt(&self.as_zstr(), f)
  }
}
impl core::fmt::Debug for ZString {
  /// Debug formats with outer `"` around the string.
  ///
  /// ```rust
  /// # use zstring::*;
  /// let zstring = ZString::try_from("foo").unwrap();
  /// let s = format!("{zstring:?}");
  /// assert_eq!("\"foo\"", s);
  /// ```
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(&self.as_zstr(), f)
  }
}
impl core::fmt::Pointer for ZString {
  /// Formats the wrapped pointer value.
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Pointer::fmt(&self.as_zstr(), f)
  }
}
