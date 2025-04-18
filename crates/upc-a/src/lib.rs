//! # upc-a
//!
//! A Rust library for parsing, validating, and working with [UPC-A (Universal Product Code)](https://www.gs1us.org/upcs-barcodes-prefixes/guide-to-upcs).
//!
//! ## What is UPC-A?
//!
//! UPC-A is a 12-digit number used to identify a product. Each UPC-A number consists of 11 digits
//! plus a check digit. The check digit is calculated using a specific algorithm based on the first
//! 11 digits.
//!
//! ## Features
//!
//! - Memory-efficient representation (8 bytes)
//! - Format-aware serialization/deserialization with [serde]
//! - Binary serialization support via [bitcode]
//! - Comprehensive error handling for invalid input
//! - No-std compatible, zero heap allocation
//!
//! [serde]: https://docs.rs/serde
//! [bitcode]: https://docs.rs/bitcode
//!
//! ## Usage
//!
//! ```rust
//! use upc_a::UpcA;
//! use std::str::FromStr;
//!
//! // From numeric UPC-A code
//! let upc_a = UpcA::from_code(123456789012)?;
//! # assert_eq!(upc_a.to_code(), 123456789012);
//!
//! // From string representation using FromStr trait
//! let upc_a = UpcA::from_str("123456789012")?;
//! # assert_eq!(upc_a.to_code(), 123456789012);
//!
//! // From a compact binary format
//! let upc_a = UpcA::from_bytes(b"\x14\x1A\x99\xBE\x1C")?;
//!
//! // Get the UPC-A code as a numeric value
//! assert_eq!(upc_a.to_code(), 123456789012);
//!
//! // Display a UPC-A code
//! assert_eq!(upc_a.to_string(), "123456789012");
//!
//! // Convert to a compact binary format
//! assert_eq!(upc_a.to_bytes(), *b"\x14\x1A\x99\xBE\x1C");
//! # anyhow::Ok::<()>(())
//! ```
//!
//! ### Serde integration
//!
//! ```rust
//! # #[cfg(feature = "serde")]
//! # {
//! use upc_a::UpcA;
//! use serde::{Deserialize, Serialize};
//!
//! // Define a struct with a UPC-A code
//! #[derive(Serialize, Deserialize)]
//! struct Product {
//!     name: String,
//!     upc_a: UpcA,
//! }
//!
//! // For human-readable formats like JSON and TOML, ISRCs are serialized as strings
//! let json = r#"{"name":"Meme cat plush","upc_a":123456789012}"#;
//! let product: Product = serde_json::from_str(json)?;
//! # assert_eq!(product.upc_a.to_code(), 123456789012);
//! # }
//! # anyhow::Ok::<()>(())
//! ```

#![no_std]
#![deny(missing_docs)]

use core::fmt::{self, Display, Formatter};
use core::num::ParseIntError;
use core::str::FromStr;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use thiserror::Error;

/// Universal Product Code version A (UPC-A)
///
/// A UPC-A code uniquely identifies a product using a 12-digit number. The last digit is a check
/// digit.
///
/// # Examples
///
/// ```
/// use upc_a::UpcA;
///
/// // Parse an ISRC from a string
/// let upc_a = UpcA::from_code(123456789012)?;
///
/// // Retrieve the numeric value
/// assert_eq!(upc_a.to_code(), 123456789012);
///
/// // Display a formatted ISRC
/// assert_eq!(upc_a.to_string(), "123456789012");
/// # anyhow::Ok::<()>(())
/// ```
///
/// ###### References
/// - <https://en.wikipedia.org/wiki/Universal_Product_Code>
/// - <https://www.gtin.info/upc/>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "bitcode", derive(Encode, Decode))]
pub struct UpcA(
    /// 12-digit number. The last digit is a error detecting check digit, but interestingly,
    /// whether we include the last digit or not, 5 bytes are still required, so we include the
    /// last digit.
    ///
    /// - Without error detecting check digit: 37 bits = 5 bytes (2**36 < 10**11 < 2**37)
    /// - With error detecting check digit: 40 bits = 5 bytes (2**39 < 10**12 < 2**40)
    u64,
);

#[test]
fn test_upc_a_size() {
    assert_eq!(size_of::<UpcA>(), 8);
}

/// Error that can occur during parsing a UPC-A code.
///
/// This enum represents all the possible errors that can occur when validating or parsing
/// a UPC-A from various input formats.
#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum UpcAParseError {
    /// The input is too large to be a valid UPC-A code.
    #[error("Input is too large (expected 0 <= input <= 999_999_999_999, found {found})")]
    InputTooLarge {
        /// The (invalid) input value that was too large.
        found: u64,
    },

    /// The input string is not a valid integer.
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    /// The checksum digit is invalid.
    #[error("Checksum failed (expected 0, found {found})")]
    ChecksumFailed {
        /// The (invalid) checksum digit that was found.
        found: u8,
    },
}

impl UpcA {
    /// Creates an [`UpcA`] from a numeric code.
    ///
    /// The input must be a 12-digit number, and the last digit must be a valid checksum digit.
    ///
    /// # Examples
    ///
    /// ```
    /// use upc_a::UpcA;
    ///
    /// // Valid UPC-A
    /// let upc_a = UpcA::from_code(123456789012)?;
    /// assert_eq!(upc_a.to_string(), "123456789012");
    ///
    /// // Invalid UPC-A (incorrect checksum)
    /// assert!(UpcA::from_code(123456789010).is_err());
    /// # anyhow::Ok::<()>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `UpcAParseError` if:
    /// - The integer value exceeds the maximum allowed value (999,999,999,999)
    /// - The checksum digit is invalid
    pub const fn from_code(n: u64) -> Result<Self, UpcAParseError> {
        if n > 999_999_999_999 {
            return Err(UpcAParseError::InputTooLarge { found: n });
        }

        // Checksum
        let mut a = n;
        let mut odd_sum = 0;
        let mut even_sum = 0;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;
        a /= 10;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;
        a /= 10;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;
        a /= 10;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;
        a /= 10;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;
        a /= 10;
        even_sum += a % 10;
        a /= 10;
        odd_sum += a % 10;

        let checksum = ((odd_sum * 3 + even_sum) % 10) as u8;
        if checksum != 0 {
            return Err(UpcAParseError::ChecksumFailed { found: checksum });
        }

        Ok(Self(n))
    }

    /// Returns the decimal integer value of a [`UpcA`].
    ///
    /// # Examples
    ///
    /// ```
    /// use upc_a::UpcA;
    /// use std::str::FromStr;
    ///
    /// let upc_a = UpcA::from_str("123456789012")?;
    /// assert_eq!(upc_a.to_code(), 123456789012);
    /// # anyhow::Ok::<()>(())
    /// ```
    pub const fn to_code(self) -> u64 {
        self.0
    }

    /// Creates a [`UpcA`] from a 5-byte binary representation.
    ///
    /// This method deserializes a UPC-A code from its compact binary representation, which is
    /// primarily useful for binary serialization formats.
    ///
    /// # Examples
    ///
    /// ```
    /// use upc_a::UpcA;
    ///
    /// let bytes = [0x14, 0x1A, 0x99, 0xBE, 0x1C];
    /// let upc_a = UpcA::from_bytes(&bytes)?;
    /// assert_eq!(upc_a.to_code(), 123456789012);
    /// # anyhow::Ok::<()>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an `UpcAParseError` if:
    /// - The integer value exceeds the maximum allowed value (999,999,999,999)
    /// - The checksum digit is invalid
    pub const fn from_bytes(bytes: &[u8; 5]) -> Result<Self, UpcAParseError> {
        let ret = u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0]);
        Self::from_code(ret)
    }

    /// Converts the [`UpcA`] to its compact 5-byte binary representation.
    ///
    /// This method serializes a UPC-A code into a fixed-size array suitable for binary storage or
    /// transmission. It is the inverse of `from_bytes`.
    ///
    /// # Examples
    ///
    /// ```
    /// use upc_a::UpcA;
    /// use std::str::FromStr;
    ///
    /// let upc_a = UpcA::from_code(123456789012)?;
    /// let bytes = upc_a.to_bytes();
    /// assert_eq!(bytes, *b"\x14\x1A\x99\xBE\x1C");
    ///
    /// // Round-trip conversion
    /// let round_trip = UpcA::from_bytes(&bytes)?;
    /// assert_eq!(round_trip, upc_a);
    /// # anyhow::Ok::<()>(())
    /// ```
    pub const fn to_bytes(self) -> [u8; 5] {
        [
            self.0 as u8,
            (self.0 >> 8) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 24) as u8,
            (self.0 >> 32) as u8,
        ]
    }
}

/// Implements the [`FromStr`] trait for [`UpcA`] to allow parsing from strings using the `parse`
/// method.
///
/// This implementation delegates to [`UpcA::from_code`].
///
/// # Examples
///
/// ```
/// use upc_a::UpcA;
/// use std::str::FromStr;
///
/// // Parse using FromStr
/// let upc_a = UpcA::from_str("123456789012")?;
/// # assert_eq!(upc_a.to_code(), 123456789012);
///
/// // Or using the more idiomatic parse method
/// let upc_a: UpcA = "123456789012".parse()?;
/// # assert_eq!(upc_a.to_code(), 123456789012);
/// # anyhow::Ok::<()>(())
/// ```
impl FromStr for UpcA {
    type Err = UpcAParseError;

    fn from_str(s: &str) -> Result<Self, UpcAParseError> {
        Self::from_code(s.parse()?)
    }
}

/// Implements the [`Display`] trait for [`UpcA`] to provide a string representation.
///
/// # Examples
///
/// ```
/// use upc_a::UpcA;
///
/// let upc_a = UpcA::from_code(123456789012)?;
/// assert_eq!(upc_a.to_string(), "123456789012");
/// # anyhow::Ok::<()>(())
/// ```
impl Display for UpcA {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:012}", self.0)
    }
}

#[test]
#[allow(
    clippy::zero_prefixed_literal,
    reason = "A UPC is a 12-digit decimal number that can start with a zero."
)]
fn test_upc_a() -> Result<(), UpcAParseError> {
    #[cfg(feature = "alloc")]
    use alloc::string::ToString;

    let upc = UpcA::from_code(123456789012)?;
    assert_eq!(upc.0, 123456789012);
    #[cfg(feature = "alloc")]
    assert_eq!(upc.to_string(), "123456789012");

    let upc: UpcA = UpcA::from_code(036000291452)?;
    assert_eq!(upc.0, 036000291452);
    #[cfg(feature = "alloc")]
    assert_eq!(upc.to_string(), "036000291452");

    assert_matches::assert_matches!(
        UpcA::from_code(1_000_000_000_000),
        Err(UpcAParseError::InputTooLarge {
            found: 1_000_000_000_000
        })
    );

    assert_matches::assert_matches!(
        UpcA::from_code(999_999_999_999),
        Err(UpcAParseError::ChecksumFailed { found: 6 })
    );

    assert_matches::assert_matches!(
        "036000291453".parse::<UpcA>(),
        Err(UpcAParseError::ChecksumFailed { found: 1 })
    );

    Ok(())
}

/// Implements the [`Serialize`] trait for [`UpcA`] to support serialization with serde.
///
/// This implementation provides format-aware serialization:
/// - For human-readable formats (like JSON, TOML): Uses a numeric representation ([`UpcA::to_code`])
/// - For binary formats (like bincode): Uses the binary representation ([`UpcA::to_bytes`])
///
/// # Examples
///
/// ```
/// use upc_a::UpcA;
/// use std::str::FromStr;
///
/// let upc_a = UpcA::from_code(123456789012)?;
///
/// // JSON serialization (human-readable)
/// let json = serde_json::to_string(&upc_a)?;
/// assert_eq!(json, r#"123456789012"#);
///
/// // Bincode serialization (binary)
/// let binary = bincode::serialize(&upc_a)?;
/// # anyhow::Ok::<()>(())
/// ```
#[cfg(feature = "serde")]
impl Serialize for UpcA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            self.0.serialize(serializer)
        } else {
            self.to_bytes().serialize(serializer)
        }
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_upc_a_serialize() -> anyhow::Result<()> {
    let upc = UpcA::from_code(123456789012)?;

    // JSON (human readable)
    assert_eq!(serde_json::to_string(&upc)?, r#"123456789012"#);
    // TOML (human readable)
    #[cfg(feature = "alloc")]
    {
        use alloc::collections::BTreeMap;
        let table: BTreeMap<&str, UpcA> = BTreeMap::from_iter([("upc", upc)]);
        assert_eq!(
            toml::to_string(&table)?,
            r#"upc = 123456789012
"#
        );
    }
    // Bincode (binary)
    assert_eq!(
        bincode::serialize(&upc)?,
        // little endian representation of 0x1cbe991a14
        b"\x14\x1A\x99\xBE\x1C"
    );

    Ok(())
}

/// Implements the [`Deserialize`] trait for [`UpcA`] to support deserialization with serde.
///
/// This implementation provides format-aware deserialization:
/// - For human-readable formats (like JSON, TOML): Expects a number and uses [`UpcA::from_code`]
/// - For binary formats (like bincode): Expects an 5-byte array and uses [`UpcA::from_bytes`]
///
/// # Examples
///
/// ```
/// use upc_a::UpcA;
/// use serde::Deserialize;
///
/// // JSON deserialization (human-readable)
/// #[derive(Deserialize)]
/// struct Product {
///     code: UpcA,
/// }
///
/// let product: Product = serde_json::from_str(r#"{"code":123456789012}"#)?;
/// # assert_eq!(product.code.to_code(), 123456789012);
/// # anyhow::Ok::<()>(())
/// ```
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for UpcA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            UpcA::from_code(u64::deserialize(deserializer)?)
        } else {
            UpcA::from_bytes(&<[u8; 5]>::deserialize(deserializer)?)
        }
        .map_err(serde::de::Error::custom)
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_upc_a_deserialize() -> anyhow::Result<()> {
    #[derive(Deserialize)]
    struct TestInput {
        upc: UpcA,
    }

    // JSON (human readable)
    let v: TestInput = serde_json::from_str(r#"{"upc": 123456789012}"#)?;
    assert_eq!(v.upc.0, 123456789012);
    // TOML (human readable)
    let v: TestInput = toml::from_str(r#"upc = 123456789012"#)?;
    assert_eq!(v.upc.0, 123456789012);
    // Bincode (binary)
    let v: UpcA = bincode::deserialize(b"\x14\x1A\x99\xBE\x1C")?;
    assert_eq!(v.0, 123456789012);

    Ok(())
}
