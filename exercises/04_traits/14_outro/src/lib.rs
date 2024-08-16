// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16
}

impl SaturatingU16 {
    fn new<T>(value: T) -> SaturatingU16
    where
        T: Into<u16> {
        return SaturatingU16 { value: value.into() };
    }
}

impl PartialEq<u16> for SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        return self.value.eq(other);   
    }
    fn ne(&self, other: &u16) -> bool {
        return self.value.ne(other);
    }
}

impl PartialEq<u16> for &SaturatingU16 {
    fn eq(&self, other: &u16) -> bool {
        return self.value.eq(other);   
    }
    fn ne(&self, other: &u16) -> bool {
        return self.value.ne(other);
    }
}

impl Add<SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs.value));
    }
}

impl Add<SaturatingU16> for &SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: SaturatingU16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs.value));
    }
}

impl Add<&SaturatingU16> for &SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs.value));
    }
}

impl Add<&SaturatingU16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &SaturatingU16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs.value));
    }
}

impl Add<u16> for &SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs));
    }
}

impl Add<u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: u16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(rhs));
    }
}

impl Add<&u16> for SaturatingU16 {
    type Output = SaturatingU16;
    fn add(self, rhs: &u16) -> Self::Output {
        return SaturatingU16::new(self.value.saturating_add(*rhs));
    }
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> Self {
        return SaturatingU16::new(value);
    }
}
impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        return SaturatingU16::new(value);
    }
}
impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        return SaturatingU16::new(*value);
    }
}
impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        return SaturatingU16::new(*value);
    }
}