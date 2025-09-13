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

#[derive(Debug, Clone, Copy)]
pub struct SaturatingU16 {
    value: u16,
}

impl From<u16> for SaturatingU16 {
    fn from(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}

impl From<u8> for SaturatingU16 {
    fn from(value: u8) -> Self {
        SaturatingU16 {
            value: value as u16,
        }
    }
}

impl From<&u16> for SaturatingU16 {
    fn from(value: &u16) -> SaturatingU16 {
        SaturatingU16 { value: *value }
    }
}

impl From<&u8> for SaturatingU16 {
    fn from(value: &u8) -> Self {
        SaturatingU16 {
            value: *value as u16,
        }
    }
}

impl From<&SaturatingU16> for SaturatingU16 {
    fn from(value: &SaturatingU16) -> Self {
        SaturatingU16 {
            value: (*value).value,
        }
    }
}
impl<T> std::ops::Add<T> for SaturatingU16
where
    SaturatingU16: From<T>,
{
    type Output = SaturatingU16;
    fn add(self, rhs: T) -> Self::Output {
        let rhs = SaturatingU16::from(rhs).value;
        SaturatingU16 {
            value: if self.value > u16::MAX - rhs {
                u16::MAX
            } else {
                self.value + rhs
            },
        }
    }
}

impl<T> PartialEq<T> for SaturatingU16
where
    for<'a> SaturatingU16: From<&'a T>,
{
    fn eq(&self, other: &T) -> bool {
        self.value == SaturatingU16::from(other).value
    }

    fn ne(&self, other: &T) -> bool {
        self.value != SaturatingU16::from(other).value
    }
}
