# isrc-rs

[![Crates.io](https://img.shields.io/crates/v/isrc.svg)](https://crates.io/crates/isrc)
[![Documentation](https://docs.rs/isrc/badge.svg)](https://docs.rs/isrc)
[![License](https://img.shields.io/crates/l/isrc.svg)](../../COPYRIGHT)

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
- Format-aware serialization/deserialization with [serde]
- Binary serialization support via [bitcode]
- Comprehensive error handling for invalid input
- No-std compatible, zero heap allocation

[serde]: https://docs.rs/serde
[bitcode]: https://docs.rs/bitcode

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
let isrc = Isrc::from_code("AA6Q72000047")?;

// Parse using FromStr trait
let isrc = Isrc::from_str("AA6Q72000047")?;

// From a compact binary format
let isrc = Isrc::from_bytes(b"\xAF\x84\x1E\x00\x41\x41\x0F\x22")?;

// Display a formatted ISRC
assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");

// Convert to compact code format
assert_eq!(isrc.to_code(), "AA6Q72000047");

// Binary representation
assert_eq!(isrc.to_bytes(), *b"\xAF\x84\x1E\x00\x41\x41\x0F\x22");
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
let recording: Recording = serde_json::from_str(json)?;
assert_eq!(recording.isrc.to_code(), "AA6Q72000047");

// For binary formats like bincode, ISRCs are serialized efficiently as 8-byte arrays
let binary = bincode::serialize(&recording)?;
let deserialized: Recording = bincode::deserialize(&binary)?;
assert_eq!(deserialized.isrc, recording.isrc);
```
