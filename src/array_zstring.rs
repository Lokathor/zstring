use crate::{CharDecoder, ZStr, ZStringError};

/// An array of string data that's zero terminated.
///
/// This is a newtype over a byte array, with a const generic length `N`.
///
/// ## Safety
/// * The [`as_zstr`](ArrayZString<N>::as_zstr) method assumes that there's a
///   null somewhere before the end of the array. Safe code cannot break this
///   rule, but unsafe code must be sure to use the entire array. The usable
///   capacity of the string is `N-1`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ArrayZString<const N: usize>([u8; N]);
impl<const N: usize> ArrayZString<N> {
  /// Gives a zeroed array.
  ///
  /// This is the same as [`default`](ArrayZString<N>::default), but `const fn`.
  #[inline]
  #[must_use]
  pub const fn const_default() -> Self {
    Self([0_u8; N])
  }

  /// Gets a [`ZStr`] to this data.
  ///
  /// ## Panics
  /// * If the length `N` is zero, this will panic.
  #[inline]
  #[must_use]
  pub const fn as_zstr(&self) -> ZStr<'_> {
    assert!(N > 0);
    unsafe { core::mem::transmute::<*const u8, ZStr<'_>>(self.0.as_ptr()) }
  }

  /// View the data as a rust str reference.
  #[inline]
  #[must_use]
  pub fn as_str(&self) -> &str {
    let null_position = self.0.iter().position(|&b| b == 0).unwrap();
    core::str::from_utf8(&self.0[..null_position]).unwrap()
  }

  /// An iterator over the bytes of this `ZStr`.
  ///
  /// * This iterator *excludes* the terminating 0 byte.
  #[inline]
  pub fn bytes(&self) -> impl Iterator<Item = u8> + '_ {
    self.0.iter().copied().take_while(|&b| b != 0)
  }

  /// An iterator over the decoded `char` values of this `ZStr`.
  #[inline]
  pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
    CharDecoder::from(self.bytes())
  }

  /// Gets the raw pointer to this data.
  #[inline]
  #[must_use]
  pub const fn as_ptr(self) -> *const u8 {
    self.0.as_ptr()
  }
}
impl<const N: usize> Default for ArrayZString<N> {
  #[inline]
  #[must_use]
  fn default() -> Self {
    Self::const_default()
  }
}
impl<const N: usize> TryFrom<&str> for ArrayZString<N> {
  type Error = Option<ZStringError>;
  /// Attempts to make an `ArrayZString` from a `&str`
  ///
  /// ```
  /// # use zstring::*;
  /// let arr_str: ArrayZString<16> = ArrayZString::try_from("hello").unwrap();
  /// assert_eq!(arr_str.as_str(), "hello");
  /// ```
  ///
  /// ## Failure
  /// The error type is unfortunately awkward here because 0.2 released with an
  /// exhaustive error type. So instead we get an "Option<ZStringError>", where
  /// "Some" is an actual [`ZStringError`] and "None" indicates that there was
  /// no zstring related issue, just a lack of capacity.
  ///
  /// * Any number of trailing nulls are allowed, and will be trimmed.
  /// * Interior nulls are not allowed (err:
  ///   `Some(ZStringError::InteriorNulls)`).
  /// * The trimmed byte length must be less than or equal to `N-1` (err:
  ///   `None`).
  ///
  /// ```
  /// # use zstring::*;
  /// let interior_null_err: Option<ZStringError> =
  ///   ArrayZString::<16>::try_from("hel\0lo").unwrap_err();
  /// assert_eq!(interior_null_err, Some(ZStringError::InteriorNulls));
  ///
  /// // strings equal to or greater than the array size won't fit.
  /// let capacity_err: Option<ZStringError> =
  ///   ArrayZString::<5>::try_from("hello").unwrap_err();
  /// assert_eq!(capacity_err, None);
  ///
  /// // if the array size exceeds the string size it will fit.
  /// assert!(ArrayZString::<6>::try_from("hello").is_ok());
  /// ```
  #[inline]
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let trimmed = value.trim_end_matches('\0');
    if trimmed.as_bytes().iter().copied().any(|b| b == 0) {
      Err(Some(ZStringError::InteriorNulls))
    } else if trimmed.len() <= (N - 1) {
      let mut out = Self::const_default();
      out.0[..trimmed.len()].copy_from_slice(trimmed.as_bytes());
      Ok(out)
    } else {
      Err(None)
    }
  }
}
impl<const N: usize> core::fmt::Display for ArrayZString<N> {
  /// Display formats the string (without outer `"`).
  ///
  /// ```rust
  /// # use zstring::*;
  /// let arr_str: ArrayZString<16> = ArrayZString::try_from("foo").unwrap();
  /// let s = format!("{arr_str}");
  /// assert_eq!("foo", s);
  /// ```
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Display::fmt(&self.as_zstr(), f)
  }
}
impl<const N: usize> core::fmt::Debug for ArrayZString<N> {
  /// Debug formats with outer `"` around the string.
  ///
  /// ```rust
  /// # use zstring::*;
  /// let arr_str: ArrayZString<16> = ArrayZString::try_from("foo").unwrap();
  /// let s = format!("{arr_str:?}");
  /// assert_eq!("\"foo\"", s);
  /// ```
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    core::fmt::Debug::fmt(&self.as_zstr(), f)
  }
}
