//! # isrc
//!
//! A Rust library for parsing, validating, and working with [ISRC (International Standard Recording Code)](http://isrc.ifpi.org/).
//!
//! ## What is ISRC?
//!
//! An ISRC uniquely identifies sound recordings and music videos internationally. Each ISRC consists of 12 alphanumeric characters with the following structure:
//!
//! - **Agency Code** (2 characters): Unique identifier for the ISRC agency
//! - **Registrant Code** (3 characters): Unique identifier for the ISRC registrant
//! - **Year of Reference** (2 digits): Identifies the year the ISRC was assigned
//! - **Designation Code** (5 digits): Uniquely assigned number by the registrant
//!
//! When formatted for display, an ISRC typically appears as: `ISRC AA-RRR-YY-DDDDD`
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
//! use isrc::Isrc;
//! use std::str::FromStr;
//!
//! // Parse an ISRC from a string
//! let isrc = Isrc::from_code("AA6Q72000047")?;
//! # assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
//!
//! // Parse using FromStr trait
//! let isrc = Isrc::from_str("AA6Q72000047")?;
//! # assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
//!
//! // From a compact binary format
//! let isrc = Isrc::from_bytes(b"\xAF\x84\x1E\x00\x41\x41\x0F\x22")?;
//! # assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
//!
//! // Display a formatted ISRC
//! assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");
//!
//! // Convert to compact code format
//! # #[cfg(feature = "alloc")]
//! assert_eq!(isrc.to_code(), "AA6Q72000047");
//!
//! // Binary representation
//! assert_eq!(isrc.to_bytes(), *b"\xAF\x84\x1E\x00\x41\x41\x0F\x22");
//! # anyhow::Ok::<()>(())
//! ```
//!
//! ### Serde integration
//!
//! ```rust
//! # #[cfg(feature = "serde")]
//! # {
//! use isrc::Isrc;
//! use serde::{Deserialize, Serialize};
//!
//! // Define a struct with an ISRC field
//! #[derive(Serialize, Deserialize)]
//! struct Recording {
//!     title: String,
//!     isrc: Isrc,
//! }
//!
//! // For human-readable formats like JSON and TOML, ISRCs are serialized as strings
//! let json = r#"{"title":"Some Song","isrc":"AA6Q72000047"}"#;
//! let recording: Recording = serde_json::from_str(json)?;
//! # assert_eq!(recording.isrc.to_code_fixed(), *b"AA6Q72000047");
//!
//! // For binary formats like bincode, ISRCs are serialized efficiently as 8-byte arrays
//! let binary = bincode::serialize(&recording)?;
//! let deserialized: Recording = bincode::deserialize(&binary)?;
//! assert_eq!(deserialized.isrc, recording.isrc);
//! # }
//! # anyhow::Ok::<()>(())
//! ```

#![no_std]
#![deny(missing_docs)]

use core::fmt::{self, Display, Formatter};
use core::str::{FromStr, from_utf8_unchecked};

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "bitcode")]
use bitcode::{Decode, Encode};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use thiserror::Error;

/// International Standard Recording Code (ISRC)
///
/// An ISRC uniquely identifies sound recordings and music videos internationally.
/// Each ISRC consists of 12 alphanumeric characters with the following structure:
///
/// - Agency Code (2 characters): Unique identifier for the ISRC agency
/// - Registrant Code (3 characters): Unique identifier for the ISRC registrant
/// - Year of Reference (2 digits): Identifies the year the ISRC was assigned
/// - Designation Code (5 digits): Uniquely assigned number by the registrant
///
/// When formatted for display, an ISRC may appear as: `ISRC AA-RRR-YY-DDDDD`
///
/// # Examples
///
/// ```
/// use isrc::Isrc;
///
/// // Parse an ISRC from a string
/// let isrc = Isrc::from_code("AA6Q72000047")?;
///
/// // Display a formatted ISRC
/// assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");
///
/// // Convert to compact code format
/// # #[cfg(feature = "alloc")]
/// assert_eq!(isrc.to_code(), "AA6Q72000047");
/// # anyhow::Ok::<()>(())
/// ```
///
/// ###### References
/// - <https://isrc.ifpi.org/en/isrc-standard/structure>
/// - <https://www.ifpi.org/wp-content/uploads/2021/02/ISRC_Handbook.pdf>
/// - <https://isrc.ifpi.org/downloads/Valid_Characters.pdf>
/// - <https://isrc.ifpi.org/downloads/ISRC_Bulletin-2015-01.pdf>
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[cfg_attr(feature = "bitcode", derive(Encode, Decode))]
pub struct Isrc {
    agency_prefix: [u8; 2],
    registrant_prefix: u16,
    rest: u32,
}

#[test]
fn test_isrc_size() {
    assert_eq!(size_of::<Isrc>(), 8);
}

/// Error that can occur when parsing an ISRC from string or bytes.
///
/// This enum represents all the possible errors that can occur when validating or parsing
/// an ISRC from various input formats.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Error)]
#[cfg_attr(feature = "bitcode", derive(Encode, Decode))]
pub enum IsrcParseError {
    /// The input string has an invalid length. An ISRC code must be exactly 12 characters.
    #[error("Invalid length (expected 12B input, found {found}B)")]
    InvalidLength {
        /// The (invalid) length of the input string.
        found: usize,
    },

    /// The agency code contains an invalid character. Must be ASCII letters [a-zA-Z].
    #[error(r"Invalid agency prefix (expected [a-zA-Z], found '\x{found:x}' at {pos})")]
    InvalidAgencyPrefix {
        /// The (invalid) character found in the agency code.
        found: u8,
        /// The position of the invalid character in the input string.
        pos: u8,
    },

    /// The registrant code contains an invalid character. Must be alphanumeric [a-zA-Z0-9].
    #[error(r"Invalid registrant prefix (expected [a-zA-Z0-9], found '\x{found:x}' at {pos})")]
    InvalidRegistrantPrefix {
        /// The (invalid) character found in the registrant code.
        found: u8,
        /// The position of the invalid character in the input string.
        pos: u8,
    },

    /// The registrant prefix exceeds the maximum allowed value (must be < 36³).
    #[error(r"Registrant prefix out of range (expected 0 <= value < 36*36*36, found {value})")]
    RegistrantPrefixOutOfRange {
        /// The (invalid) value of the registrant prefix.
        value: u16,
    },

    /// A digit in the year or designation code is invalid. Must be numeric [0-9].
    #[error(r"Invalid digit (expected [0-9], found '\x{found:x}' at {pos})")]
    InvalidDigit {
        /// The (invalid) character found in the year or designation code.
        found: u8,
        /// The position of the invalid character in the input string.
        pos: u8,
    },

    /// The numeric portion of the ISRC exceeds the maximum allowed value (must be < 10,000,000).
    #[error(r"Rest value out of range (expected 0 <= value < 10000000, found {value})")]
    ValueOutOfRange {
        /// The (invalid) value of the numeric portion of the ISRC.
        value: u32,
    },
}

impl Isrc {
    /// Creates an [`Isrc`] from a string code.
    ///
    /// The input string must be exactly 12 characters long and follow the ISRC format:
    /// - First 2 characters: Agency code (letters only)
    /// - Next 3 characters: Registrant code (alphanumeric)
    /// - Last 7 characters: Year and designation code (digits only)
    ///
    /// Letters in the input string are case-insensitive and will be normalized to uppercase.
    ///
    /// # Examples
    ///
    /// ```
    /// use isrc::Isrc;
    ///
    /// // Valid ISRC
    /// let isrc = Isrc::from_code("AA6Q72000047")?;
    /// assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");
    ///
    /// // Valid ISRC with lowercase letters
    /// let isrc = Isrc::from_code("aa6q72000047")?;
    /// assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");
    ///
    /// // Invalid ISRC (incorrect length)
    /// assert!(Isrc::from_code("AA6Q7200047").is_err());
    /// # anyhow::Ok::<()>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`IsrcParseError`] if:
    /// - The input is not exactly 12 characters long
    /// - The agency code contains non-letter characters
    /// - The registrant code contains characters that are not alphanumeric
    /// - The year and designation codes contain non-digit characters
    /// - The values exceed their allowed ranges
    pub const fn from_code(code: &str) -> Result<Self, IsrcParseError> {
        let code = code.as_bytes();
        if code.len() != 12 {
            return Err(IsrcParseError::InvalidLength { found: code.len() });
        }

        macro_rules! agency {
            ($pos:expr) => {
                match code[$pos] {
                    b'a'..=b'z' => code[$pos] ^ 0b0010_0000, // to_ascii_uppercase
                    b'A'..=b'Z' => code[$pos],
                    _ => {
                        return Err(IsrcParseError::InvalidAgencyPrefix {
                            found: code[$pos],
                            pos: $pos,
                        })
                    }
                }
            };
        }
        macro_rules! registrant {
            ($pos:expr) => {
                match code[$pos] {
                    b'0'..=b'9' => code[$pos] - b'0',
                    b'a'..=b'z' => code[$pos] - b'a' + 10,
                    b'A'..=b'Z' => code[$pos] - b'A' + 10,
                    _ => return Err(IsrcParseError::InvalidRegistrantPrefix { found: code[$pos], pos: $pos }),
                } as u16
            };
        }

        macro_rules! d {
            ($pos:expr) => {
                match code[$pos] {
                    b'0'..=b'9' => code[$pos] - b'0',
                    _ => return Err(IsrcParseError::InvalidDigit { found: code[$pos], pos: $pos }),
                } as u32
            };
        }

        Ok(Isrc {
            agency_prefix: [agency!(0), agency!(1)],
            registrant_prefix: registrant!(2) * 36 * 36 + registrant!(3) * 36 + registrant!(4),
            rest: d!(5) * 1_000_000
                + d!(6) * 100_000
                + d!(7) * 10_000
                + d!(8) * 1_000
                + d!(9) * 100
                + d!(10) * 10
                + d!(11),
        })
    }

    /// Converts the [`Isrc`] to its compact 12-character string representation.
    ///
    /// This method returns the ISRC in its compact format without hyphens or the "ISRC" prefix,
    /// which is suitable for storage or transmission.
    ///
    /// # Examples
    ///
    /// ```
    /// use isrc::Isrc;
    /// use std::str::FromStr;
    ///
    /// let isrc = Isrc::from_str("AA6Q72000047")?;
    /// assert_eq!(isrc.to_code(), "AA6Q72000047");
    /// # anyhow::Ok::<()>(())
    /// ```
    #[cfg(feature = "alloc")]
    pub fn to_code(&self) -> String {
        let mut n = self.registrant_prefix;

        let d2 = ascii_uppercase_from_digit_base36(n) as char;
        n /= 36;

        let d1 = ascii_uppercase_from_digit_base36(n) as char;
        n /= 36;

        let d0 = ascii_uppercase_from_digit_base36(n) as char;

        alloc::format!(
            "{}{}{}{}{:07}",
            // SAFETY: `self.agency_prefix` is always ASCII uppercase
            unsafe { from_utf8_unchecked(&self.agency_prefix) },
            d0,
            d1,
            d2,
            self.rest,
        )
    }

    /// Converts the [`Isrc`] to its compact 12-character string representation. It's zero heap
    /// allocation version of [`Isrc::to_code`].
    ///
    /// This method returns the ISRC in its compact format without hyphens or the "ISRC" prefix,
    /// which is suitable for storage or transmission.
    ///
    /// # Examples
    ///
    /// ```
    /// use isrc::Isrc;
    /// use std::str::FromStr;
    ///
    /// let isrc = Isrc::from_str("AA6Q72000047")?;
    /// assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
    /// # anyhow::Ok::<()>(())
    /// ```
    pub const fn to_code_fixed(&self) -> [u8; 12] {
        use core::mem::MaybeUninit;

        let mut ret = [const { MaybeUninit::<u8>::uninit() }; 12];

        // ret[5..12] = sprintf("%07d", self.rest)
        let mut n = self.rest;
        ret[11].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[10].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[9].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[8].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[7].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[6].write(b'0' + (n % 10) as u8);
        n /= 10;
        ret[5].write(b'0' + (n % 10) as u8);

        let mut n = self.registrant_prefix;
        ret[4].write(ascii_uppercase_from_digit_base36(n));
        n /= 36;
        ret[3].write(ascii_uppercase_from_digit_base36(n));
        n /= 36;
        ret[2].write(ascii_uppercase_from_digit_base36(n));

        ret[1].write(self.agency_prefix[1]);
        ret[0].write(self.agency_prefix[0]);

        // SAFETY: The array is fully initialized
        unsafe { core::mem::transmute::<_, [u8; 12]>(ret) }
    }

    /// Creates an [`Isrc`] from an 8-byte array.
    ///
    /// This method deserializes an ISRC from its compact binary representation, which is
    /// primarily useful for binary serialization formats.
    ///
    /// The binary layout is:
    /// - Bytes 0-3: `rest` field (year and designation code) as little-endian u32
    /// - Bytes 4-5: `agency_prefix` as two ASCII uppercase letters
    /// - Bytes 6-7: `registrant_prefix` as little-endian u16
    ///
    /// # Examples
    ///
    /// ```
    /// use isrc::Isrc;
    ///
    /// let bytes = [0xB1, 0xCB, 0x74, 0x00, b'Z', b'Z', 0x0F, 0x22];
    /// let isrc = Isrc::from_bytes(&bytes)?;
    /// # #[cfg(feature = "alloc")]
    /// assert_eq!(isrc.to_code(), "ZZ6Q77654321");
    /// # anyhow::Ok::<()>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an [`IsrcParseError`] if:
    /// - The agency prefix contains non-uppercase letter characters
    /// - The registrant prefix exceeds the maximum allowed value (36³-1)
    /// - The rest field exceeds the maximum allowed value (9,999,999)
    pub const fn from_bytes(bytes: &[u8; 8]) -> Result<Self, IsrcParseError> {
        // Stored according to the default Rust struct layout.
        let rest = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        let agency_prefix = [bytes[4], bytes[5]];
        let registrant_prefix = u16::from_le_bytes([bytes[6], bytes[7]]);

        if !agency_prefix[0].is_ascii_uppercase() {
            return Err(IsrcParseError::InvalidAgencyPrefix {
                found: agency_prefix[0],
                pos: 4,
            });
        }
        if !agency_prefix[1].is_ascii_uppercase() {
            return Err(IsrcParseError::InvalidAgencyPrefix {
                found: agency_prefix[1],
                pos: 5,
            });
        }
        if registrant_prefix >= 36 * 36 * 36 {
            return Err(IsrcParseError::RegistrantPrefixOutOfRange {
                value: registrant_prefix,
            });
        }
        if rest >= 10_000_000 {
            return Err(IsrcParseError::ValueOutOfRange { value: rest });
        }

        Ok(Isrc {
            agency_prefix,
            registrant_prefix,
            rest,
        })
    }

    /// Converts the [`Isrc`] to its compact 8-byte binary representation.
    ///
    /// This method serializes the ISRC into a fixed-size array suitable for binary storage
    /// or transmission. It is the inverse of [`Isrc::from_bytes`].
    ///
    /// # Examples
    ///
    /// ```
    /// use isrc::Isrc;
    /// use std::str::FromStr;
    ///
    /// let isrc = Isrc::from_str("AZ6Q77654321")?;
    /// let bytes = isrc.to_bytes();
    /// assert_eq!(bytes, [0xB1, 0xCB, 0x74, 0x00, b'A', b'Z', 0x0F, 0x22]);
    ///
    /// // Round-trip conversion
    /// let round_trip = Isrc::from_bytes(&bytes)?;
    /// assert_eq!(round_trip, isrc);
    /// # anyhow::Ok::<()>(())
    /// ```
    pub const fn to_bytes(&self) -> [u8; 8] {
        [
            self.rest as u8,
            (self.rest >> 8) as u8,
            (self.rest >> 16) as u8,
            (self.rest >> 24) as u8,
            self.agency_prefix[0],
            self.agency_prefix[1],
            self.registrant_prefix as u8,
            (self.registrant_prefix >> 8) as u8,
        ]
    }
}

#[test]
fn test_isrc() -> Result<(), IsrcParseError> {
    let isrc = Isrc::from_code("AA6Q72000047")?;
    assert_eq!(
        isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    #[cfg(feature = "alloc")]
    assert_eq!(isrc.to_code(), "AA6Q72000047");
    assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");

    // Lowercase
    let isrc = Isrc::from_code("aa6q72000047")?;
    assert_eq!(
        isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    #[cfg(feature = "alloc")]
    assert_eq!(isrc.to_code(), "AA6Q72000047");
    assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");

    // Invalid inputs
    assert_eq!(
        Isrc::from_code("00A6Q7200047"),
        Err(IsrcParseError::InvalidAgencyPrefix {
            found: b'0',
            pos: 0
        })
    );

    assert_eq!(
        Isrc::from_code("AA-6Q7200047"),
        Err(IsrcParseError::InvalidRegistrantPrefix {
            found: b'-',
            pos: 2
        })
    );

    assert_eq!(
        Isrc::from_code("aa6q7200047\n"),
        Err(IsrcParseError::InvalidDigit {
            found: b'\n',
            pos: 11
        })
    );

    Ok(())
}

#[test]
fn test_isrc_from_bytes() -> Result<(), IsrcParseError> {
    let isrc = Isrc::from_bytes(&[0xB1, 0xCB, 0x74, 0x00, 0x5A, 0x5A, 0x0F, 0x22])?;
    assert_eq!(
        isrc,
        Isrc {
            agency_prefix: [b'Z', b'Z'],
            registrant_prefix: 8719,
            rest: 76_54321,
        }
    );
    #[cfg(feature = "alloc")]
    assert_eq!(isrc.to_code(), "ZZ6Q77654321");
    assert_eq!(isrc.to_code_fixed(), *b"ZZ6Q77654321");

    // Invalid inputs
    let fail = Isrc::from_bytes(&[0xB1, 0xCB, 0x74, 0x00, 0x5A, 0x00, 0x0F, 0x22]);
    assert_eq!(
        fail,
        Err(IsrcParseError::InvalidAgencyPrefix {
            found: 0x00,
            pos: 5
        })
    );

    let fail = Isrc::from_bytes(&[0xB1, 0xCB, 0x74, 0x00, 0x5A, 0x5A, 0xFF, 0xFF]);
    assert_eq!(
        fail,
        Err(IsrcParseError::RegistrantPrefixOutOfRange { value: 0xFFFF })
    );

    let fail = Isrc::from_bytes(&[0xFF, 0xFF, 0xFF, 0xFF, 0x5A, 0x5A, 0x0F, 0x22]);
    assert_eq!(
        fail,
        Err(IsrcParseError::ValueOutOfRange { value: 0xFFFFFFFF })
    );

    Ok(())
}

#[test]
fn test_isrc_to_bytes() {
    assert_eq!(
        Isrc {
            agency_prefix: [b'A', b'Z'],
            registrant_prefix: 8719,
            rest: 76_54321,
        }
        .to_bytes(),
        [0xB1, 0xCB, 0x74, 0x00, b'A', b'Z', 0x0F, 0x22]
    );
}

/// Implements the [`FromStr`] trait for [`Isrc`] to allow parsing from strings using the `parse`
/// method.
///
/// This implementation delegates to [`Isrc::from_code`].
///
/// # Examples
///
/// ```
/// use isrc::Isrc;
/// use std::str::FromStr;
///
/// // Parse using FromStr
/// let isrc = Isrc::from_str("AA6Q72000047")?;
/// # assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
///
/// // Or using the more idiomatic parse method
/// let isrc: Isrc = "AA6Q72000047".parse()?;
/// # assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");
/// # anyhow::Ok::<()>(())
/// ```
impl FromStr for Isrc {
    type Err = IsrcParseError;

    fn from_str(code: &str) -> Result<Self, IsrcParseError> {
        Isrc::from_code(code)
    }
}

#[test]
fn test_isrc_from_str() -> Result<(), IsrcParseError> {
    let isrc: Isrc = "AA6Q72000047".parse()?;
    assert_eq!(
        isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    #[cfg(feature = "alloc")]
    assert_eq!(isrc.to_code(), "AA6Q72000047");
    assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");

    // Lowercase
    let isrc: Isrc = "aa6q72000047".parse()?;
    assert_eq!(
        isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    #[cfg(feature = "alloc")]
    assert_eq!(isrc.to_code(), "AA6Q72000047");
    assert_eq!(isrc.to_code_fixed(), *b"AA6Q72000047");

    // Invalid inputs
    assert_eq!(
        "00A6Q7200047".parse::<Isrc>(),
        Err(IsrcParseError::InvalidAgencyPrefix {
            found: b'0',
            pos: 0
        })
    );

    assert_eq!(
        "AA6Q7200047".parse::<Isrc>(),
        Err(IsrcParseError::InvalidLength { found: 11 })
    );

    assert_eq!(
        "AA6Q7200047910".parse::<Isrc>(),
        Err(IsrcParseError::InvalidLength { found: 14 })
    );

    Ok(())
}

/// Implements the [`Display`] trait for [`Isrc`] to provide a formatted string representation.
///
/// This formats the ISRC with the standard hyphenated format and "ISRC" prefix:
/// `ISRC AA-RRR-YY-NNNNN` where:
/// - AA is the agency code
/// - RRR is the registrant code
/// - YY is the year of reference
/// - NNNNN is the designation code
///
/// # Examples
///
/// ```
/// use isrc::Isrc;
/// use std::str::FromStr;
///
/// let isrc = Isrc::from_str("AA6Q72000047")?;
/// assert_eq!(isrc.to_string(), "ISRC AA-6Q7-20-00047");
/// # anyhow::Ok::<()>(())
/// ```
impl Display for Isrc {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut n = self.registrant_prefix;

        let d2 = ascii_uppercase_from_digit_base36(n % 36) as char;
        n /= 36;

        let d1 = ascii_uppercase_from_digit_base36(n % 36) as char;
        n /= 36;

        let d0 = ascii_uppercase_from_digit_base36(n % 36) as char;

        write!(
            f,
            "ISRC {}-{}{}{}-{:02}-{:05}",
            // SAFETY: `self.agency_prefix` is always ASCII uppercase
            unsafe { from_utf8_unchecked(&self.agency_prefix) },
            d0,
            d1,
            d2,
            self.rest / 100_000,
            self.rest % 100_000,
        )
    }
}

#[test]
fn test_isrc_display() -> anyhow::Result<()> {
    use core::fmt::Write;

    let mut buffer = heapless::String::<32>::new();
    let isrc = Isrc::from_code("AA6Q72000047")?;
    write!(&mut buffer, "{}", isrc)?;
    assert_eq!(buffer, "ISRC AA-6Q7-20-00047");

    let mut buffer = heapless::String::<20>::new();
    let isrc = Isrc::from_code("Zz6q72412345")?;
    write!(&mut buffer, "{}", isrc)?;
    assert_eq!(buffer, "ISRC ZZ-6Q7-24-12345");

    Ok(())
}

/// Implements the [`Serialize`] trait for [`Isrc`] to support serialization with serde.
///
/// This implementation provides format-aware serialization:
/// - For human-readable formats (like JSON, TOML): Uses the compact string representation ([`Isrc::to_code`])
/// - For binary formats (like bincode): Uses the binary representation ([`Isrc::to_bytes`])
///
/// # Examples
///
/// ```
/// use isrc::Isrc;
/// use std::str::FromStr;
///
/// let isrc = Isrc::from_str("AA6Q72000047")?;
///
/// // JSON serialization (human-readable)
/// let json = serde_json::to_string(&isrc)?;
/// assert_eq!(json, r#""AA6Q72000047""#);
///
/// // Bincode serialization (binary)
/// let binary = bincode::serialize(&isrc)?;
/// # anyhow::Ok::<()>(())
/// ```
#[cfg(feature = "serde")]
impl Serialize for Isrc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use core::str::from_utf8_unchecked;

        if serializer.is_human_readable() {
            let code = self.to_code_fixed();

            // SAFETY: `to_code_fixed` always returns a valid UTF-8 string
            unsafe { from_utf8_unchecked(&code) }.serialize(serializer)
        } else {
            self.to_bytes().serialize(serializer)
        }
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_isrc_serialize() -> anyhow::Result<()> {
    let isrc = Isrc::from_code("AA6Q72000047")?;

    // JSON (human readable)
    assert_eq!(serde_json::to_string(&isrc)?, r#""AA6Q72000047""#);
    // TOML (human readable)
    #[cfg(feature = "alloc")]
    {
        use alloc::collections::BTreeMap;
        let table: BTreeMap<&str, Isrc> = BTreeMap::from_iter([("isrc", isrc)]);
        assert_eq!(
            toml::to_string(&table)?,
            r#"isrc = "AA6Q72000047"
"#
        );
    }
    // Bincode (binary)
    assert_eq!(
        bincode::serialize(&isrc)?,
        b"\xAF\x84\x1E\x00\x41\x41\x0f\x22"
    );

    Ok(())
}

/// Implements the [`Deserialize`] trait for [`Isrc`] to support deserialization with serde.
///
/// This implementation provides format-aware deserialization:
/// - For human-readable formats (like JSON, TOML): Expects a string and uses [`Isrc::from_code`]
/// - For binary formats (like bincode): Expects an 8-byte array and uses [`Isrc::from_bytes`]
///
/// # Examples
///
/// ```
/// use isrc::Isrc;
/// use serde::Deserialize;
///
/// // JSON deserialization (human-readable)
/// let isrc: Isrc = serde_json::from_str(r#""AA6Q72000047""#)?;
/// # #[cfg(feature = "alloc")]
/// assert_eq!(isrc.to_code(), "AA6Q72000047");
///
/// // From a struct
/// #[derive(Deserialize)]
/// struct Record {
///     isrc: Isrc,
/// }
///
/// let record: Record = serde_json::from_str(r#"{"isrc":"AA6Q72000047"}"#)?;
/// # #[cfg(feature = "alloc")]
/// assert_eq!(record.isrc.to_code(), "AA6Q72000047");
/// # anyhow::Ok::<()>(())
/// ```
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Isrc {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            Isrc::from_code(&heapless::String::<12>::deserialize(deserializer)?)
        } else {
            Isrc::from_bytes(&<[u8; 8]>::deserialize(deserializer)?)
        }
        .map_err(serde::de::Error::custom)
    }
}

#[test]
#[cfg(feature = "serde")]
fn test_isrc_deserialize() -> anyhow::Result<()> {
    #[derive(Debug, Deserialize)]
    struct TestInput {
        isrc: Isrc,
    }

    // JSON (human readable)
    let v: TestInput = serde_json::from_str(r#"{"isrc":"AA6Q72000047"}"#)?;
    assert_eq!(
        v.isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    // TOML (human readable)
    let v: TestInput = toml::from_str(r#"isrc = "AA6Q72000047""#)?;
    assert_eq!(
        v.isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );
    // Bincode (binary)
    let v: TestInput = bincode::deserialize(b"\xAF\x84\x1E\x00\x41\x41\x0f\x22")?;
    assert_eq!(
        v.isrc,
        Isrc {
            agency_prefix: [b'A', b'A'],
            registrant_prefix: 8719,
            rest: 20_00047,
        }
    );

    //
    // Wrong inputs
    //
    // JSON (human readable)
    let r: Result<TestInput, _> = serde_json::from_str(r#"{"isrc":"AA6Q72000047777777"}"#);
    let Err(serde_json::Error { .. }) = r else {
        panic!("Expected error, but got: {r:?}");
    };
    // TOML (human readable)
    let r: Result<TestInput, _> = toml::from_str(r#"isrc = "AA6Q72000""#);
    let Err(toml::de::Error { .. }) = r else {
        panic!("Expected error, but got: {r:?}");
    };
    // Bincode (binary)
    let r: Result<TestInput, _> = bincode::deserialize(b"\xAF\x84\x00\x41\x41\x0f\x22");
    let Err(bincode::Error { .. }) = r else {
        panic!("Expected error, but got: {r:?}");
    };

    Ok(())
}

const fn ascii_uppercase_from_digit_base36(d: u16) -> u8 {
    let d = (d % 36) as u8;
    match d {
        0..=9 => d + b'0',
        10..=35 => d + b'A' - 10,
        _ => unreachable!(),
    }
}
