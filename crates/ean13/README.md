# ean-13

<!--
[![Crates.io](https://img.shields.io/crates/v/ean13.svg)](https://crates.io/crates/ean13)
[![Documentation](https://docs.rs/ean13/badge.svg)](https://docs.rs/ean13)
[![License](https://img.shields.io/crates/l/ean13.svg)](../../COPYRIGHT)
-->

A Rust library for parsing, validating, and working with [EAN-13](https://www.gs1.org/standards/barcodes/ean-upc).

## What is EAN-13?

EAN-13 is a 13-digit number used to identify a product internationally. EAN-13
is an extension of the earlier 12-digit UPC-A with an optional numeric prefix
indicating the country of registration.

## Features

- Memory-efficient representation (8 bytes)
- Format-aware serialization/deserialization with [serde]
- Binary serialization support via [bitcode]
- Comprehensive error handling for invalid input
- No-std compatible, zero heap allocation

[serde]: https://docs.rs/serde
[bitcode]: https://docs.rs/bitcode

## Usage

```rust
use ean13::Ean13;
use std::str::FromStr;

// From numeric UPC-A code
let ean13 = Ean13::from_code(4006381333931)?;

// From string representation using FromStr trait
let ean13 = Ean13::from_str("4006381333931")?;

// From a compact binary format
let ean13 
let upc_a = UpcA::from_bytes(b"\x14\x1A\x99\xBE\x1C")?;

// Get the UPC-A code as a numeric value
assert_eq!(upc_a.to_code(), 123456789012);

// Display a UPC-A code
assert_eq!(upc_a.to_string(), "123456789012");

// Convert to a compact binary format
assert_eq!(upc_a.to_bytes(), *b"\x14\x1A\x99\xBE\x1C");
```

### Serde integration

```rust
use upc_a::UpcA;
use serde::{Deserialize, Serialize};

// Define a struct with a UPC-A code
#[derive(Serialize, Deserialize)]
struct Product {
    name: String,
    upc_a: UpcA,
}

// For human-readable formats like JSON and TOML, ISRCs are serialized as strings
let json = r#"{"name":"Meme cat plush","upc_a":123456789012}"#;
let product: Product = serde_json::from_str(json)?;
```
