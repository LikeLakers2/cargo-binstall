use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};

/// Binary format enumeration
#[derive(
    Debug, Display, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, EnumString, EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(ascii_case_insensitive)]
pub enum PkgFmt {
    /// Download format is TAR (uncompressed)
    Tar,
    /// Download format is TAR + Bzip2
    Tbz2,
    /// Download format is Bzip2
    Bz2,
    /// Download format is TGZ (TAR + GZip)
    Tgz,
    /// Download format is GZ (GZip)
    Gz,
    /// Download format is TAR + XZ
    Txz,
    /// Download format is XZ
    Xz,
    /// Download format is TAR + Zstd
    Tzstd,
    /// Download format is ZST (zstd)
    Zst,
    /// Download format is Zip
    Zip,
    /// Download format is raw / binary
    Bin,
}

impl Default for PkgFmt {
    fn default() -> Self {
        Self::Tgz
    }
}

impl PkgFmt {
    /// If self is one of the tar based formats, return Some.
    pub fn decompose(self) -> PkgFmtDecomposed {
        match self {
            PkgFmt::Tar => PkgFmtDecomposed::Tar(TarBasedFmt::Tar),
            PkgFmt::Tbz2 => PkgFmtDecomposed::Tar(TarBasedFmt::Tbz2),
            PkgFmt::Bz2 => PkgFmtDecomposed::Bz2,
            PkgFmt::Tgz => PkgFmtDecomposed::Tar(TarBasedFmt::Tgz),
            PkgFmt::Gz => PkgFmtDecomposed::Gz,
            PkgFmt::Txz => PkgFmtDecomposed::Tar(TarBasedFmt::Txz),
            PkgFmt::Xz => PkgFmtDecomposed::Xz,
            PkgFmt::Tzstd => PkgFmtDecomposed::Tar(TarBasedFmt::Tzstd),
            PkgFmt::Zst => PkgFmtDecomposed::Zst,
            PkgFmt::Bin => PkgFmtDecomposed::Bin,
            PkgFmt::Zip => PkgFmtDecomposed::Zip,
        }
    }

    /// List of possible file extensions for the format
    /// (with prefix `.`).
    ///
    /// * `is_windows` - if true and `self == PkgFmt::Bin`, then it will return
    ///   `.exe` in additional to other bin extension names.
    pub fn extensions(self, is_windows: bool) -> &'static [&'static str] {
        match self {
            PkgFmt::Tar => &[".tar"],
            PkgFmt::Tbz2 => &[".tbz2", ".tar.bz2"],
            PkgFmt::Bz2 => &[".bz2"],
            PkgFmt::Tgz => &[".tgz", ".tar.gz"],
            PkgFmt::Gz => &[".gz"],
            PkgFmt::Txz => &[".txz", ".tar.xz"],
            PkgFmt::Xz => &[".xz"],
            PkgFmt::Tzstd => &[".tzstd", ".tzst", ".tar.zst"],
            PkgFmt::Zst => &[".zst"],
            PkgFmt::Bin => {
                if is_windows {
                    &[".bin", "", ".exe"]
                } else {
                    &[".bin", ""]
                }
            }
            PkgFmt::Zip => &[".zip"],
        }
    }

    /// Given the pkg-url template, guess the possible pkg-fmt.
    pub fn guess_pkg_format(pkg_url: &str) -> Option<Self> {
        let mut it = pkg_url.rsplitn(3, '.');

        let guess = match it.next()? {
            "tar" => Some(PkgFmt::Tar),

            "tbz2" => Some(PkgFmt::Tbz2),
            "bz2" => Some(PkgFmt::Bz2),

            "tgz" => Some(PkgFmt::Tgz),
            "gz" => Some(PkgFmt::Gz),

            "txz" => Some(PkgFmt::Txz),
            "xz" => Some(PkgFmt::Xz),

            "tzstd" | "tzst" => Some(PkgFmt::Tzstd),
            "zst" => Some(PkgFmt::Zst),

            "exe" | "bin" => Some(PkgFmt::Bin),
            "zip" => Some(PkgFmt::Zip),

            _ => None,
        };

        // If we have a guess, and our next segment is "tar"...
        if guess.is_some() && it.next() == Some("tar") {
            // ...And if there's another segment before it...
            if it.next().is_some() {
                // ...then we have a `.tar.{fmt}`, so we convert our guess a tar-based format
                guess.map(|pkgfmt| match pkgfmt {
                    PkgFmt::Bz2 => PkgFmt::Tbz2,
                    PkgFmt::Gz => PkgFmt::Tgz,
                    PkgFmt::Xz => PkgFmt::Txz,
                    PkgFmt::Zst => PkgFmt::Tzstd,
                    _ => pkgfmt,
                })
            } else {
                // Otherwise, we can assume our pkg_url to be malformed
                None
            }
        } else {
            // Otherwise, assume our guess is correct.
            guess
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PkgFmtDecomposed {
    Tar(TarBasedFmt),
    Bz2,
    Gz,
    Xz,
    Zst,
    Bin,
    Zip,
}

#[derive(Debug, Display, Copy, Clone, Eq, PartialEq)]
pub enum TarBasedFmt {
    /// Download format is TAR (uncompressed)
    Tar,
    /// Download format is TAR + Bzip2
    Tbz2,
    /// Download format is TGZ (TAR + GZip)
    Tgz,
    /// Download format is TAR + XZ
    Txz,
    /// Download format is TAR + Zstd
    Tzstd,
}

impl From<TarBasedFmt> for PkgFmt {
    fn from(fmt: TarBasedFmt) -> Self {
        match fmt {
            TarBasedFmt::Tar => PkgFmt::Tar,
            TarBasedFmt::Tbz2 => PkgFmt::Tbz2,
            TarBasedFmt::Tgz => PkgFmt::Tgz,
            TarBasedFmt::Txz => PkgFmt::Txz,
            TarBasedFmt::Tzstd => PkgFmt::Tzstd,
        }
    }
}
