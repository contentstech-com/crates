#![no_std]

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
/// ###### References
/// - https://en.wikipedia.org/wiki/Universal_Product_Code
/// - https://www.gtin.info/upc/
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

#[derive(Clone, Eq, PartialEq, Debug, Error)]
pub enum UpcAParseError {
    #[error("Input is too large (expected 0 <= input <= 999_999_999_999, found {found})")]
    InputTooLarge { found: u64 },
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Checksum failed (expected 0, found {found})")]
    ChecksumFailed { found: u8 },
}

impl UpcA {
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

    pub const fn from_bytes(bytes: &[u8; 5]) -> Result<Self, UpcAParseError> {
        let ret = u64::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], 0, 0, 0]);
        Self::from_code(ret)
    }

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

impl FromStr for UpcA {
    type Err = UpcAParseError;

    fn from_str(s: &str) -> Result<Self, UpcAParseError> {
        Self::from_code(s.parse()?)
    }
}

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
