# isrc-rs

[![Crates.io](https://img.shields.io/crates/v/isrc.svg)](https://crates.io/crates/isrc)
[![Documentation](https://docs.rs/isrc/badge.svg)](https://docs.rs/isrc)
[![License](https://img.shields.io/crates/l/isrc.svg)][COPYRIGHT]

A Rust library for parsing, validating, and working with [ISRC (International Standard Recording Code)](http://isrc.ifpi.org/).

## What is ISRC?

An ISRC uniquely identifies sound recordings and music videos internationally. Each ISRC consists of 12 alphanumeric characters with the following structure:

- **Agency Code** (2 characters): Unique identifier for the ISRC agency
- **Registrant Code** (3 characters): Unique identifier for the ISRC registrant
- **Year of Reference** (2 digits): Identifies the year the ISRC was assigned
- **Designation Code** (5 digits): Uniquely assigned number by the registrant

When formatted for display, an ISRC typically appears as: `ISRC AA-RRR-YY-DDDDD`

## Features

- Memory-efficient representation (8 bytes)
- Format-aware serialization/deserialization with `serde`
- Binary serialization support via `bitcode`
- Comprehensive error handling for invalid input
- No-std compatible, zero heap allocation

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
isrc = "0.1"
```

### Basic examples

```rust
use isrc::Isrc;
use std::str::FromStr;

// Parse an ISRC from a string
let isrc = Isrc::from_code("AA6Q72000047").unwrap();

// Parse using FromStr trait (with proper error handling)
let result = Isrc::from_str("AA6Q72000047");
match result {
    Ok(isrc) => println!("Valid ISRC: {}", isrc),
    Err(e) => println!("Invalid ISRC: {}", e),
}

// Display a formatted ISRC
assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");

// Convert to compact code format
assert_eq!(isrc.to_code(), "AA6Q72000047");

// Binary representation
let bytes = isrc.to_bytes();
let isrc_from_bytes = Isrc::from_bytes(&bytes).unwrap();
assert_eq!(isrc, isrc_from_bytes);
```

### Serde integration

```rust
use isrc::Isrc;
use serde::{Deserialize, Serialize};

// Define a struct with an ISRC field
#[derive(Serialize, Deserialize)]
struct Recording {
    title: String,
    isrc: Isrc,
}

// For human-readable formats like JSON and TOML, ISRCs are serialized as strings
let json = r#"{"title":"Some Song","isrc":"AA6Q72000047"}"#;
let recording: Recording = serde_json::from_str(json).unwrap();
assert_eq!(recording.isrc.to_code(), "AA6Q72000047");

// For binary formats like bincode, ISRCs are serialized efficiently as 8-byte arrays
let binary = bincode::serialize(&recording).unwrap();
let deserialized: Recording = bincode::deserialize(&binary).unwrap();
assert_eq!(deserialized.isrc, recording.isrc);
```

&nbsp;

---

*isrc-rs* is primarily distributed under the terms of both the [Apache License
(Version 2.0)] and the [MIT license]. See [COPYRIGHT] for details.

[MIT license]: ../../LICENSE-MIT
[Apache License (Version 2.0)]: ../../LICENSE-APACHE
[COPYRIGHT]: ../../COPYRIGHT
