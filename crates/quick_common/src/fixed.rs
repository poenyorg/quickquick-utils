// fixed.rs
// ===============================
// Fixed-point arithmetic for trading system
// Global scale = 1e8
// Wire: int64
// Core: i128
// ===============================

use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub, SubAssign};
use std::str::FromStr;
use std::string::ParseError;

use rust_decimal::Decimal;

const SCALE: i128 = 100_000_000;
const SCALE_I64: i64 = 100_000_000;
const SCALE_DIGITS: u32 = 8;
const ZERO: Fixed = Fixed(0);
const ONE: Fixed = Fixed(SCALE);
const TWO: Fixed = Fixed(2 * SCALE);
const TEN: Fixed = Fixed(10 * SCALE);
const ONE_HUNDRED: Fixed = Fixed(100 * SCALE);
const ONE_THOUSAND: Fixed = Fixed(1000 * SCALE);
const MIN: Fixed = Fixed(i64::MIN as i128 * SCALE);
const MAX: Fixed = Fixed(i64::MAX as i128 * SCALE);

/// Wrapper kiểu fixed-point (1e8) sử dụng nội bộ là i128
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Fixed(pub i128);

impl Fixed {
    pub const SCALE: i128 = SCALE;
    pub const SCALE_I64: i64 = SCALE_I64;
    pub const SCALE_DIGITS: u32 = SCALE_DIGITS;
    pub const ZERO: Fixed = ZERO;
    pub const ONE: Fixed = ONE;
    pub const TWO: Fixed = TWO;
    pub const TEN: Fixed = TEN;
    pub const ONE_HUNDRED: Fixed = ONE_HUNDRED;
    pub const ONE_THOUSAND: Fixed = ONE_THOUSAND;
    pub const MIN: Fixed = MIN;
    pub const MAX: Fixed = MAX;

    pub fn value(&self) -> i128 {
        self.0
    }
    /// Tạo từ wire i64 (gRPC)
    #[inline(always)]
    pub fn from_wire(v: i64) -> Self {
        Fixed(v as i128)
    }

    pub fn from_i64(v: i64) -> Self {
        Fixed((v * SCALE_I64) as i128)
    }

    /// Tạo từ f64 (CHỈ dùng debug / test / nhập liệu)
    #[allow(dead_code)]
    pub fn from_f64(v: f64) -> Self {
        Fixed((v * SCALE as f64).round() as i128)
    }

    pub fn from_decimal(d: Decimal) -> Fixed {
        // Chuẩn hoá Decimal về scale = 8 (1e8)
        let mut c_d = d;
        c_d.rescale(SCALE_DIGITS);

        // Lấy mantissa i128 trực tiếp
        Fixed(c_d.mantissa())
    }

    pub fn from_opt_str(s: &Option<String>) -> Self {
        if s.is_none() {
            Fixed(0)
        } else {
            Self::from_str(&s.clone().unwrap()).unwrap()
        }
    }

    pub fn from_opt_str_opt(s: &Option<String>) -> Option<Self> {
        if s.is_none() {
            None
        } else {
            Some(Self::from_str(&s.clone().unwrap()).unwrap())
        }
    }

    /// Convert sang f64 (CHỈ dùng hiển thị)
    #[allow(dead_code)]
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / SCALE as f64
    }

    /// Convert về wire i64 để gửi gRPC
    #[inline(always)]
    pub fn to_wire(self) -> i64 {
        if self.0 > i64::MAX as i128 || self.0 < i64::MIN as i128 {
            panic!("Fixed overflow when converting to i64 wire");
        }
        self.0 as i64
    }

    pub fn to_decimal(&self) -> Decimal {
        Decimal::from_i128_with_scale(self.0, SCALE_DIGITS)
    }

    /// Multiply fixed-point
    #[inline]
    pub fn mul_fixed_point(self, rhs: Fixed) -> Fixed {
        Fixed((self.0 * rhs.0) / SCALE)
    }

    /// Divide fixed-point
    #[inline]
    pub fn div_fixed_point(self, rhs: Fixed) -> Fixed {
        Fixed((self.0 * SCALE) / rhs.0)
    }

    pub fn percent_of(self, percent: Fixed) -> Fixed {
        Fixed((self.0 * percent.0) / (SCALE * 100))
    }

    pub fn round_to(self, tick: Fixed, is_up: bool) -> Fixed {
        if is_up {
            self.round_up(tick)
        } else {
            self.round_down(tick)
        }
    }

    /// Rounding theo tick size
    pub fn round_up(self, tick: Fixed) -> Fixed {
        let remainder = self.0 % tick.0;
        if remainder != 0 {
            return Fixed(self.0 + tick.0 - remainder);
        }
        self
    }

    pub fn round_down(self, tick: Fixed) -> Fixed {
        let remainder = self.0 % tick.0;
        if remainder != 0 {
            return Fixed(self.0 - remainder);
        }
        self
    }

    /// Abs
    pub fn abs(self) -> Fixed {
        Fixed(self.0.abs())
    }

    /// Zero
    pub fn zero() -> Fixed {
        Fixed(0)
    }

    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    pub fn is_positive(self) -> bool {
        self.0 > 0
    }

    pub fn is_negative(self) -> bool {
        self.0 < 0
    }

    pub fn reverse(self) -> Fixed {
        Fixed(-self.0)
    }

    pub fn as_negative(self) -> Fixed {
        if self.0.is_positive() {
            return Fixed(-self.0);
        }
        self
    }

    pub fn swap_neg(self, is_neg: bool) -> Fixed {
        if is_neg && self.0.is_positive() {
            return Fixed(-self.0);
        }
        self
    }
}

// ===========================
// Trait Implementations
// ===========================

impl FromStr for Fixed {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let negative = s.starts_with('-');
        let s = s.trim_start_matches('-');

        let parts: Vec<&str> = s.split('.').collect();
        let int: i128 = parts[0].parse().unwrap();

        let frac = if parts.len() > 1 {
            let mut frac = parts[1].to_string();
            frac.truncate(8);
            while frac.len() < 8 {
                frac.push('0');
            }
            frac.parse::<i128>().unwrap()
        } else {
            0
        };

        let v = int * SCALE + frac;
        Ok(Self(if negative { -v } else { v }))
    }
}

impl Add for Fixed {
    type Output = Fixed;

    fn add(self, rhs: Fixed) -> Fixed {
        Fixed(self.0 + rhs.0)
    }
}

impl Sub for Fixed {
    type Output = Fixed;

    fn sub(self, rhs: Fixed) -> Fixed {
        Fixed(self.0 - rhs.0)
    }
}

impl Mul<i128> for Fixed {
    type Output = Fixed;

    fn mul(self, rhs: i128) -> Fixed {
        Fixed(self.0 * rhs)
    }
}

impl Div<i128> for Fixed {
    type Output = Fixed;

    fn div(self, rhs: i128) -> Fixed {
        Fixed(self.0 / rhs)
    }
}

impl SubAssign for Fixed {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Fixed) {
        self.0 -= rhs.0;
    }
}

impl AddAssign for Fixed {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Fixed) {
        self.0 += rhs.0;
    }
}

impl Sum for Fixed {
    #[inline]
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Fixed::zero(), |acc, x| Fixed(acc.0 + x.0))
    }
}

impl<'a> Sum<&'a Fixed> for Fixed {
    #[inline]
    fn sum<I: Iterator<Item = &'a Fixed>>(iter: I) -> Self {
        iter.fold(Fixed::zero(), |acc, x| Fixed(acc.0 + x.0))
    }
}

impl fmt::Debug for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let neg = self.0 < 0;
        let abs = self.0.abs();

        let int = abs / SCALE;
        let frac = abs % SCALE;

        let out = format!("{}{}.{:08}", if neg { "-" } else { "" }, int, frac);
        write!(f, "{}", out)
    }
}

// ===========================
// Vì muda ghi panic, nên nên wrap các checked ops nếu muốn production strict
// ===========================

impl Fixed {
    pub fn checked_add(self, rhs: Fixed) -> Option<Fixed> {
        self.0.checked_add(rhs.0).map(Fixed)
    }

    pub fn checked_sub(self, rhs: Fixed) -> Option<Fixed> {
        self.0.checked_sub(rhs.0).map(Fixed)
    }

    pub fn checked_mul(self, rhs: Fixed) -> Option<Fixed> {
        self.0
            .checked_mul(rhs.0)
            .and_then(|v| v.checked_div(SCALE))
            .map(Fixed)
    }

    pub fn checked_div(self, rhs: Fixed) -> Option<Fixed> {
        self.0
            .checked_mul(SCALE)
            .and_then(|v| v.checked_div(rhs.0))
            .map(Fixed)
    }
}

// ===========================
// Custom Serialization cho Fixed
// ===========================

impl Serialize for Fixed {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as string để giữ độ chính xác
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Fixed {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(FixedVisitor)
    }
}

struct FixedVisitor;

impl<'de> Visitor<'de> for FixedVisitor {
    type Value = Fixed;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string, integer, or float representing a fixed-point number")
    }

    // Deserialize từ string
    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Fixed::from_str(value)
            .map_err(|_| E::custom(format!("invalid fixed-point string: {}", value)))
    }

    // Deserialize từ i64
    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Fixed::from_wire(value))
    }

    // Deserialize từ u64
    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if value > i64::MAX as u64 {
            return Err(E::custom(format!("u64 value {} exceeds i64::MAX", value)));
        }
        Ok(Fixed::from_wire(value as i64))
    }

    // Deserialize từ f64 (ít chính xác hơn, chỉ dùng khi cần)
    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Fixed::from_f64(value))
    }
}

// ===========================
// Các module serialize tùy chỉnh
// ===========================

/// Module để serialize/deserialize Fixed as i64 (wire format)
pub mod fixed_as_i64 {
    use super::*;

    pub fn serialize<S>(value: &Fixed, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(value.to_wire())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Fixed, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = i64::deserialize(deserializer)?;
        Ok(Fixed::from_wire(v))
    }
}

/// Module để serialize/deserialize Fixed as string
pub mod fixed_as_string {
    use super::*;

    pub fn serialize<S>(value: &Fixed, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&value.to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Fixed, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Fixed::from_str(&s).map_err(de::Error::custom)
    }
}

/// Module để serialize/deserialize Option<Fixed> as i64
pub mod option_fixed_as_i64 {
    use super::*;

    pub fn serialize<S>(value: &Option<Fixed>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(f) => serializer.serialize_some(&f.to_wire()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Fixed>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<i64>::deserialize(deserializer)?;
        Ok(opt.map(Fixed::from_wire))
    }
}

/// Module để serialize/deserialize Option<Fixed> as string
pub mod option_fixed_as_string {
    use super::*;

    pub fn serialize<S>(value: &Option<Fixed>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(f) => serializer.serialize_some(&f.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Fixed>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => Fixed::from_str(&s).map(Some).map_err(de::Error::custom),
            None => Ok(None),
        }
    }
}

// ===========================
// TEST
// ===========================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_format() {
        let x = Fixed::from_str("65000.00012345").unwrap();
        assert_eq!(x.to_string(), "65000.00012345");
    }

    #[test]
    fn test_reverse() {
        let x = Fixed::from_str("100").unwrap();
        let y = Fixed::from_str("-100").unwrap();
        assert_eq!(Fixed::from_i64(-100), x.reverse());
        assert_eq!(Fixed::from_i64(100), y.reverse());
    }

    #[test]
    fn test_mul() {
        let a = Fixed::from_str("100.0").unwrap();
        let b = Fixed::from_str("0.5").unwrap();
        let c = a.mul_fixed_point(b);
        assert_eq!(c.to_string(), "50.00000000");
    }

    #[test]
    fn test_wire() {
        let x = Fixed::from_str("42.1").unwrap();
        let wire = x.to_wire();
        let back = Fixed::from_wire(wire);
        assert_eq!(x, back);
    }

    #[test]
    fn test_round_up() {
        let p = Fixed::from_str("100.12345678").unwrap();
        let tick = Fixed::from_str("0.01").unwrap();
        let r = p.round_up(tick);
        assert_eq!(r.to_string(), "100.13000000");
    }

    #[test]
    fn test_round_down() {
        let p = Fixed::from_str("100.12345678").unwrap();
        let tick = Fixed::from_str("0.01").unwrap();
        let r = p.round_down(tick);
        assert_eq!(r.to_string(), "100.12000000");
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Example {
        // Default: serialize as string
        price: Fixed,

        // Explicit: serialize as i64 (wire format)
        #[serde(with = "fixed_as_i64")]
        quantity: Fixed,

        // Optional field as string
        #[serde(with = "option_fixed_as_string")]
        fee: Option<Fixed>,

        // Optional field as i64
        #[serde(with = "option_fixed_as_i64")]
        commission: Option<Fixed>,
    }

    #[test]
    fn test_serialize_deserialize() {
        let example = Example {
            price: Fixed::from_str("123.456").unwrap(),
            quantity: Fixed::from_str("10.5").unwrap(),
            fee: Some(Fixed::from_str("0.01").unwrap()),
            commission: None,
        };

        // Serialize to JSON
        let json = serde_json::to_string(&example).unwrap();
        println!("JSON: {}", json);

        // Deserialize from JSON
        let parsed: Example = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.price, example.price);
        assert_eq!(parsed.quantity, example.quantity);
        assert_eq!(parsed.fee, example.fee);
        assert_eq!(parsed.commission, example.commission);
    }

    #[test]
    fn test_deserialize_from_int() {
        // Test deserialize từ integer (wire format)
        let json = r#"{"price":"123.456","quantity":1050000000,"fee":"0.01","commission":null}"#;
        let parsed: Example = serde_json::from_str(json).unwrap();

        assert_eq!(parsed.quantity, Fixed::from_str("10.5").unwrap());
        assert_eq!(parsed.price, Fixed::from_str("123.456").unwrap());
        assert_eq!(parsed.fee, Some(Fixed::from_str("0.01").unwrap()));
        assert_eq!(parsed.commission, None);
    }
}
