use std::path::Path;
use std::fs;
use std::io;

/// Size used to align the data buffer
pub const ALIGN_SIZE: i32 = 0;

/// Minimum block size
pub const BLOCK_SIZE: i32 = 4096;

/// DirectIO extensions to [`std::fs::File`].
///
/// Thoses extensions are implemented for each OS family which are hidden
/// behind the same API interface.
///
/// [`std::fs::file`]: https://doc.rust-lang.org/std/fs/struct.File.html
pub trait DirectFileExt {
  /// Open a file in read-only mode with the `O_DIRECT` flag.
  ///
  /// See [`DirectOpenOptionsExt::direct`] for more details.
  ///
  /// # Errors
  ///
  /// This function is returning the same errors of [`std::fs::File::open`].
  ///
  /// # Examples
  ///
  /// ```no_run
  /// use directio::DirectFileExt;
  /// use std::fs::File;
  ///
  /// fn main() -> std::io::Result<()> {
  ///     let mut f = fs::direct_open("foo.txt")?;
  ///     Ok(())
  /// }
  /// ```
  ///
  /// [`DirectOpenOptionsExt::direct`]: trait.DirectOpenOptionsExt.html#tymethod.direct
  /// [`std::fs::File::open`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.open
  fn direct_open<P>(path: P) -> io::Result<fs::File>
  where
      P: AsRef<Path>;
}

/// DirectIO extensions to [`std::fs::OpenOptions`].
///
/// Thoses extensions are implemented for each OS family which are hidden
/// behind the same API interface.
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
  /// let mut options = OpenOptions::new();
  /// options.direct();
  ///
  /// let file = options.open("foo.txt");
  ///
  /// ```
  fn direct(&mut self) -> &mut Self;
}

