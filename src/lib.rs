#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "hal")]
pub mod hal;

#[cfg(feature = "ntdll")]
pub mod ntdll;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;

#[cfg(feature = "ole32")]
pub mod ole32;