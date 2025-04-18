# upc-a

[![Crates.io](https://img.shields.io/crates/v/upc-a.svg)](https://crates.io/crates/upc-a)
[![Documentation](https://docs.rs/upc-a/badge.svg)](https://docs.rs/upc-a)
[![License](https://img.shields.io/crates/l/upc-a.svg)](../../COPYRIGHT)

A Rust library for parsing, validating, and working with [UPC-A (Universal Product Code)](https://www.gs1us.org/upcs-barcodes-prefixes/guide-to-upcs).

## What is UPC-A?

UPC-A is a 12-digit number used to identify a product. Each UPC-A number consists of 11 digits
plus a check digit. The check digit is calculated using a specific algorithm based on the first
11 digits.

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
use upc_a::UpcA;
use std::str::FromStr;

// From numeric UPC-A code
let upc_a = UpcA::from_code(123456789012)?;

// From string representation using FromStr trait
let upc_a = UpcA::from_str("123456789012")?;

// From a compact binary format
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
