#![no_std]

#[cfg(feature = "host")]
#[doc(hidden)]
pub use tracing as __tracing;

#[cfg(feature = "embedded")]
#[doc(hidden)]
pub use defmt as __defmt;

#[cfg(feature = "host")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__tracing::debug!($($arg)*);
    };
}

#[cfg(feature = "embedded")]
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::__defmt::debug!($($arg)*);
    };
}

#[cfg(feature = "host")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__tracing::info!($($arg)*);
    };
}

#[cfg(feature = "embedded")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::__defmt::info!($($arg)*);
    };
}

#[cfg(feature = "host")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__tracing::warn!($($arg)*);
    };
}

#[cfg(feature = "embedded")]
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::__defmt::warn!($($arg)*);
    };
}

#[cfg(feature = "host")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__tracing::error!($($arg)*);
    };
}

#[cfg(feature = "embedded")]
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::__defmt::error!($($arg)*);
    };
}
