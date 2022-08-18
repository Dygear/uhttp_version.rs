//! This crate provides a parser/formatter for the [HTTP version
//! field](https://tools.ietf.org/html/rfc7230#section-2.6) found in the
//! request/response [start line](https://tools.ietf.org/html/rfc7230#section-3.1).
//!
//! ## Example
//!
//! ```rust
//! use uhttp_version::HttpVersion;
//! use std::io::Write;
//!
//! assert_eq!(HttpVersion::from_bytes(b"HTTP/1.0"), Ok(HttpVersion::from_parts(1, 0)));
//! assert_eq!(HttpVersion::from_bytes(b"HTTP/1.1"), Ok(HttpVersion::from_parts(1, 1)));
//! assert_eq!(HttpVersion::from_bytes(b"HTTP/4.2"), Ok(HttpVersion::from_parts(4, 2)));
//!
//! assert_eq!("HTTP/1.0".parse(), Ok(HttpVersion::from_parts(1, 0)));
//! assert_eq!("HTTP/1.1".parse(), Ok(HttpVersion::from_parts(1, 1)));
//! assert_eq!("HTTP/4.2".parse(), Ok(HttpVersion::from_parts(4, 2)));
//!
//! assert!(HttpVersion::from_bytes(b"http/1.1").is_err());
//! assert!(HttpVersion::from_bytes(b"HTTP/1.42").is_err());
//! assert!(HttpVersion::from_bytes(b"HTTP/1-1").is_err());
//!
//! let ver = HttpVersion::from_bytes(b"HTTP/1.1").unwrap();
//! assert_eq!(ver.major, 1);
//! assert_eq!(ver.minor, 1);
//!
//! let ver = HttpVersion::from_parts(4, 2);
//! let mut buf = [b'#'; 8];
//! write!(&mut buf[..], "{}", ver).unwrap();
//! assert_eq!(&buf[..], b"HTTP/4.2");
//! ```

/// HTTP start line version field [RFC7230§2.6].
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct HttpVersion {
    /// Major version number.
    pub major: u8,
    /// Minor version number.
    pub minor: u8,
}

impl HttpVersion {
    /// Create a new `HttpVersion` from the given major and minor version parts.
    pub fn from_parts(major: u8, minor: u8) -> Self {
        // Major and minor version numbers must be single digits [RFC7230§2.6].
        debug_assert!(major < 10 && minor < 10);

        HttpVersion { major, minor }
    }

    /// Try to parse an `HttpVersion` from the given bytes in the form required by the
    /// request line [syntax](https://tools.ietf.org/html/rfc7230#section-2.6).
    ///
    /// This verifies the syntax is well-formed and extracts the version parts.
    pub fn from_bytes(s: &[u8]) -> Result<Self, ()> {
        // Name is case sensitive [RFC7230§2.6].
        const NAME: &[u8] = b"HTTP/";

        if !s.starts_with(NAME) {
            return Err(());
        }

        let ver = &s[NAME.len()..];

        if ver.len() != 3 || ver[1] != b'.' {
            return Err(());
        }

        match (to_digit(ver[0]), to_digit(ver[2])) {
            (Some(major), Some(minor)) => Ok(HttpVersion::from_parts(major, minor)),
            _ => return Err(()),
        }
    }
}

/// Convert the given ASCII digit to a numeric digit if it's within the correct range.
fn to_digit(b: u8) -> Option<u8> {
    if b >= b'0' && b <= b'9' {
        Some(b - b'0')
    } else {
        None
    }
}

/// Writes the version string in the form required by the HTTP status line.
impl std::fmt::Display for HttpVersion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "HTTP/{}.{}", self.major, self.minor)
    }
}

impl std::str::FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        HttpVersion::from_bytes(s.as_bytes())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_http_ver() {
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1.0"), Ok(HttpVersion {
            major: 1,
            minor: 0,
        }));

        assert_eq!(HttpVersion::from_bytes(b"HTTP/1.1"), Ok(HttpVersion {
            major: 1,
            minor: 1,
        }));

        assert_eq!(HttpVersion::from_bytes(b"HTTP/0.0"), Ok(HttpVersion {
            major: 0,
            minor: 0,
        }));

        assert_eq!(HttpVersion::from_bytes(b"HTTP/0.1"), Ok(HttpVersion {
            major: 0,
            minor: 1,
        }));

        assert_eq!(HttpVersion::from_bytes(b"HTTP/9.9"), Ok(HttpVersion {
            major: 9,
            minor: 9,
        }));

        assert_eq!("HTTP/1.1".parse(), Ok(HttpVersion {
            major: 1,
            minor: 1,
        }));

        assert_eq!("http/1.1".parse::<HttpVersion>(), Err(()));

        assert_eq!(HttpVersion::from_bytes(b"http/1.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"Http/1.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTp/1.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"PTTH/1.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1.1 "), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1. "), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/@.@"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1.10"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/10.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP@1.1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1@1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"HTTP/1 1"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b"PTTHPTTHPTTH"), Err(()));
        assert_eq!(HttpVersion::from_bytes(b""), Err(()));

        let mut buf = [b'|'; 8];
        write!(&mut buf[..], "{}", HttpVersion::from_parts(1, 1)).unwrap();
        assert_eq!(&buf[..], b"HTTP/1.1");
        assert_eq!(HttpVersion::from_bytes(&buf[..]), Ok(HttpVersion::from_parts(1, 1)));
    }
}
