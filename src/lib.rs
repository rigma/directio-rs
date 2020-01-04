extern crate libc;

#[macro_use]
mod macros;

cfg_if! {
    if #[cfg(all(unix, target_os = "macos"))] {
        mod darwin;
        pub use darwin::*;
    } else if #[cfg(all(unix, not(target_os = "openbsd")))] {
        mod unix;
        pub use unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else {
        // TODO: add API here with unimplemented macros
    }
}
