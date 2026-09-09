#![no_std]

#[cfg(feature = "host")]
#[doc(hidden)]
pub use tracing as __backend;

#[cfg(feature = "embedded")]
#[doc(hidden)]
pub use defmt as __backend;

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__backend::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__backend::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__backend::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__backend::error!($($arg)*)
    };
}
