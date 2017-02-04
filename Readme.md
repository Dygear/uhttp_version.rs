# uhttp_version -- HTTP version field parser/formatter

[Documentation](https://docs.rs/uhttp_version)

This crate provides a parser/formatter for the [HTTP version
field](https://tools.ietf.org/html/rfc7230#section-2.6) found in the
request/response [start line](https://tools.ietf.org/html/rfc7230#section-3.1).

## Example

```rust
use uhttp_version::HttpVersion;
use std::io::Write;

assert_eq!(HttpVersion::from_bytes(b"HTTP/1.0"), Ok(HttpVersion::from_parts(1, 0)));
assert_eq!(HttpVersion::from_bytes(b"HTTP/1.1"), Ok(HttpVersion::from_parts(1, 1)));
assert_eq!(HttpVersion::from_bytes(b"HTTP/4.2"), Ok(HttpVersion::from_parts(4, 2)));

assert_eq!("HTTP/1.0".parse(), Ok(HttpVersion::from_parts(1, 0)));
assert_eq!("HTTP/1.1".parse(), Ok(HttpVersion::from_parts(1, 1)));
assert_eq!("HTTP/4.2".parse(), Ok(HttpVersion::from_parts(4, 2)));

assert!(HttpVersion::from_bytes(b"http/1.1").is_err());
assert!(HttpVersion::from_bytes(b"HTTP/1.42").is_err());
assert!(HttpVersion::from_bytes(b"HTTP/1-1").is_err());

let ver = HttpVersion::from_bytes(b"HTTP/1.1").unwrap();
assert_eq!(ver.major, 1);
assert_eq!(ver.minor, 1);

let ver = HttpVersion::from_parts(4, 2);
let mut buf = [b'#'; 8];
write!(&mut buf[..], "{}", ver).unwrap();
assert_eq!(&buf[..], b"HTTP/4.2");
```

## Usage

This [crate](https://crates.io/crates/uhttp_version) can be used through cargo by adding
it as a dependency in `Cargo.toml`:

```toml
[dependencies]
uhttp_version = "0.6.0"
```
and importing it in the crate root:

```rust
extern crate uhttp_version;
```
