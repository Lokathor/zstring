use crate::{CharDecoder, ZStr};

/// An array of string data that's zero termianted.
///
/// This is a newtype over a byte array, with a const generic length `N`.
///
/// ## Safety
/// * The [`as_zstr`](ArrayZString<N>::as_zstr) method assumes that there's a
///   null somewhere before the end of the array. Safe code cannot break this
///   rule, but unsafe code must be sure to use the entire array. The usable
///   capacity of the string is `N-1`.
#[derive(Clone, Copy)]
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
