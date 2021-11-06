#![no_std]
#![warn(missing_docs)]

//! A crate to make zero-termiated FFI data easier to work with.
//!
//! ## Literals
//!
//! You can use this to make "literals" for passing to C. Imagine that there's a
//! C function such as the following from the Windows API:
//!
//! ```c
//! FARPROC GetProcAddress(
//!   HMODULE hModule,
//!   LPCSTR  lpProcName
//! );
//! ```
//!
//! We can expose this in Rust like this
//! ```
//! use core::ffi::c_void;
//! use zstring::ZStr;
//!
//! #[link(name = "Kernel32")]
//! extern "system" {
//!   pub fn GetProcAddress<'a>(
//!     hModule: *mut c_void, lpProcName: ZStr<'a>,
//!   ) -> *mut c_void;
//! }
//! ```
//!
//! and then call it as follows:
//! ```
//! # use core::ffi::c_void;
//! # use zstring::ZStr;
//! # unsafe fn GetProcAddress<'a>(hModule: *mut c_void, lpProcName: ZStr<'a>) -> *mut c_void { core::ptr::null_mut() }
//! # let module = core::ptr::null_mut();
//! use zstring::zstr;
//!
//! let proc = unsafe { GetProcAddress(module, zstr!("initscr")) };
//! ```
//!
//! ## Allocations
//!
//! If the `alloc` feature is enabled then additional types which own their data
//! are provided.
//!
//! Say we have a C struct such as the following:
//! ```c
//! typedef struct VkInstanceCreateInfo {
//!   VkStructureType             sType;
//!   const void*                 pNext;
//!   VkInstanceCreateFlags       flags;
//!   const VkApplicationInfo*    pApplicationInfo;
//!   uint32_t                    enabledLayerCount;
//!   const char* const*          ppEnabledLayerNames;
//!   uint32_t                    enabledExtensionCount;
//!   const char* const*          ppEnabledExtensionNames;
//! } VkInstanceCreateInfo;
//! ```
//!
//! Normally it would very troublesome to get the data for `ppEnabledLayerNames`
//! and `ppEnabledExtensionNames` arranged. However, if we use a Vec<ZString>
//! then the pointer to the vec's data will naturally line up with what we need.

#[cfg(feature = "alloc")]
extern crate alloc;

macro_rules! impl_zbytes_fmt {
  ($imp_target:ty: $($t:ident),*) => {
    $(
      impl<'a> core::fmt::$t for $imp_target {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
          f.write_str("[")?;
          for (i, b) in self.iter().enumerate() {
            if i != 0 {
              f.write_str(", ")?;
            }
            core::fmt::$t::fmt(b, f)?;
          }
          f.write_str("]")?;
          Ok(())
        }
      }
    )*
  }
}

/// An error when you tried to make a z-bytes variant.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum ZBytesCreationError {
  /// An interior null was detected.
  InteriorNull,
  /// There was no 0 value at the end.
  NullTerminatorMissing,
}

mod macros;
pub use crate::macros::*;

mod zbytes_ref;
pub use crate::zbytes_ref::*;

mod zstr;
pub use crate::zstr::*;

mod char_decoder;
pub use crate::char_decoder::*;

#[cfg(feature = "alloc")]
mod zbytes;
#[cfg(feature = "alloc")]
pub use crate::zbytes::*;

#[cfg(feature = "alloc")]
mod zstring;
#[cfg(feature = "alloc")]
pub use crate::zstring::*;
