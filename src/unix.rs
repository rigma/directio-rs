use std::fs;

/// Size used to align the data buffer
pub const ALIGN_SIZE: i32 = 4096;

/// Minimum block size
pub const BLOCK_SIZE: i32 = 4096;

/// Unix-specific DirectIO extensions to [`std::fs::OpenOptions`].
/// 
/// [`std::fs::OpenOptions`]: https://doc.rust-lang.org/std/fs/struct.OpenOptions.html
pub trait DirectOpenOptionsExt {
    /// Add `O_DIRECT` to the `flags` argument of `open`.
    /// 
    /// By providing this flag on UNIX family, you'll be able to access to the
    /// file without passing through kernel buffering. This is slower but may 
    /// be useful when you need some data to be really on the storage disk (eg.
    /// for data replication for instance).
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::fs::OpenOptions;
    /// use directio::DirectOpenOptionsExt;
    /// 
    /// # fn main() {
    /// let mut options = OpenOptions::new();
    /// options.direct();
    /// 
    /// let file = options.open("foo.txt");
    /// # }
    /// ```
    fn direct(&mut self) -> &mut Self;
}

impl DirectOpenOptionsExt for fs::OpenOptions {
    fn direct(&mut self) -> &mut Self {
        use std::os::unix::fs::OpenOptionsExt;

        self.custom_flags(libc::O_DIRECT)
    }
}
