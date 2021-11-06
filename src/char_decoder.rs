/// Decodes byte sequences that are assumed to be utf-8.
pub struct CharDecoder<I: Iterator<Item = u8>> {
  iter: core::iter::Peekable<I>,
}
impl<I: Iterator<Item = u8>> CharDecoder<I> {
  /// Returns the next continuation bits (pre-masked) if the next byte is a
  /// continuation byte
  #[inline]
  fn next_continuation_bits(&mut self) -> Option<u32> {
    match self.iter.peek()? {
      x if x >> 6 == 0b10 => Some((self.iter.next()? as u32) & 0b111111),
      _ => None,
    }
  }
}
impl<I: Iterator<Item = u8>> From<I> for CharDecoder<I> {
  #[inline]
  #[must_use]
  fn from(i: I) -> Self {
    Self { iter: i.peekable() }
  }
}
impl<I: Iterator<Item = u8>> Iterator for CharDecoder<I> {
  type Item = char;

  #[inline]
  fn next(&mut self) -> Option<char> {
    let x = self.iter.next()? as u32;
    // if x is a single-byte value, we have an easy path.
    if x < 128 {
      //println!("1 byte");
      //println!("x = 0b{:08b}", x);
      return Some(unsafe { char::from_u32_unchecked(x as u32) });
    } else
    /* otherwise it's a very nasty code path */
    if (x >> 5) == 0b110 {
      let y = match self.next_continuation_bits() {
        Some(y) => y as u32,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let u = ((x & 0b11111) << 6) | y;
      //println!("2 bytes");
      //println!("x = 0b{:08b}", x);
      //println!("y = 0b{:08b}", y);
      //println!("u = 0b{:032b}", u);
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else if (x >> 4) == 0b1110 {
      let y = match self.next_continuation_bits() {
        Some(y) => y,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let z = match self.next_continuation_bits() {
        Some(z) => z,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let u = ((x & 0b1111) << 12) | y << 6 | z;
      //println!("3 bytes");
      //println!("x = 0b{:08b}", x);
      //println!("y = 0b{:08b}", y);
      //println!("z = 0b{:08b}", z);
      //println!("u = 0b{:032b}", u);
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else if (x >> 3) == 0b11110 {
      let y = match self.next_continuation_bits() {
        Some(y) => y,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let z = match self.next_continuation_bits() {
        Some(z) => z,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let w = match self.next_continuation_bits() {
        Some(w) => w,
        None => return Some(char::REPLACEMENT_CHARACTER),
      };
      let u = ((x & 0b111) << 18) | y << 12 | z << 6 | w;
      //println!("4 bytes");
      //println!("x = 0b{:08b}", x);
      //println!("y = 0b{:08b}", y);
      //println!("z = 0b{:08b}", z);
      //println!("w = 0b{:08b}", w);
      //println!("u = 0b{:032b}", u);
      Some(char::from_u32(u).unwrap_or(char::REPLACEMENT_CHARACTER))
    } else {
      // we shouldn't ever hit this case, but if we do, whatever.
      Some(char::REPLACEMENT_CHARACTER)
    }
  }
}
