#[macro_use]
extern crate cfg_if;
#[cfg(unix)]
extern crate libc;
#[cfg(windows)]
extern crate winapi;

cfg_if! {
    if #[cfg(all(unix, not(target_os = "openbsd")))] {
        mod unix;
        pub use unix::*;
    } else if #[cfg(windows)] {
        mod windows;
        pub use windows::*;
    } else {
        // TODO: add API here with unimplemented macros
    }
}
