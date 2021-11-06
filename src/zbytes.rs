use core::ptr::NonNull;

#[repr(transparent)]
pub struct ZBytes {
  pub(crate) nn: NonNull<u8>,
}
