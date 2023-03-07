#![forbid(unsafe_code)]

/// Decodes byte sequences as if they were utf-8.
///
/// If the bytes are not utf-8 you'll automatically get the
/// [`REPLACEMENT_CHARACTER`](char::REPLACEMENT_CHARACTER) within the output, as
/// necessary.
///
/// Construct this iterator using `from` on any other iterator over `u8`.
///
/// ```rust
/// # use zstring::CharDecoder;
/// let decoder1 = CharDecoder::from([32, 33, 34].into_iter());
/// let decoder2 = CharDecoder::from("foobar".as_bytes().iter().copied());
/// ```
pub struct CharDecoder<I: Iterator<Item = u8>> {
  iter: core::iter::Peekable<I>,
}
impl<I: Iterator<Item = u8>> From<I> for CharDecoder<I> {
  #[inline]
  #[must_use]
  fn from(i: I) -> Self {
    Self { iter: i.peekable() }
  }
}
impl<I: Iterator<Item = u8>> CharDecoder<I> {
  /// Returns the next continuation bits (pre-masked), only if the next byte is
  /// a continuation byte.
  #[inline]
  #[must_use]
  fn next_continuation_bits(&mut self) -> Option<u32> {
    match self.iter.peek()? {
      x if x >> 6 == 0b10 => Some((self.iter.next()? as u32) & 0b111111),
      _ => None,
    }
  }
}
impl<I: Iterator<Item = u8>> Iterator for CharDecoder<I> {
  type Item = char;

  #[inline]
  #[must_use]
  fn next(&mut self) -> Option<char> {
    let x = u32::from(self.iter.next()?);
    if x < 128 {
      Some(x as u8 as char)
    } else if (x >> 5) == 0b110 {
      let Some(y) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let u = ((x & 0b11111) << 6) | y;
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else if (x >> 4) == 0b1110 {
      let Some(y) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let Some(z) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let u = ((x & 0b1111) << 12) | y << 6 | z;
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else if (x >> 3) == 0b11110 {
      let Some(y) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let Some(z) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let Some(w) = self.next_continuation_bits() else {
        return Some(char::REPLACEMENT_CHARACTER);
      };
      let u = ((x & 0b111) << 18) | y << 12 | z << 6 | w;
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else {
      Some(char::REPLACEMENT_CHARACTER)
    }
  }
}
