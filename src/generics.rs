use anyhow::Result;
use std::{collections::HashMap, ops::Deref};

use serde::Deserialize;

pub const SECONDS_PER_TICK: f64 = 0.6;

pub trait NamedData: for<'a> Deserialize<'a> {
    fn get_name(&self) -> &str;
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct Fraction {
    pub dividend: i32,
    pub divisor: i32,
}

impl std::ops::Mul<Scalar> for Fraction {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar((self.dividend * rhs.0) / self.divisor)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Percentage(i32);

impl std::ops::Mul<Scalar> for Percentage {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar(((100 + self.0) * rhs.0) / 100)
    }
}

impl From<i32> for Percentage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Percentage {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Scalar(i32);

impl Scalar {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

impl std::ops::Mul for Scalar {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl std::ops::Mul<Fraction> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Fraction) -> Self::Output {
        Self((self.0 * rhs.dividend) / rhs.divisor)
    }
}

impl std::ops::Mul<Percentage> for Scalar {
    type Output = Self;

    fn mul(self, rhs: Percentage) -> Self::Output {
        Self((self.0 * (100 + rhs.0)) / 100)
    }
}

impl std::ops::Div for Scalar {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl std::ops::Add for Scalar {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for Scalar {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::SubAssign for Scalar {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl From<i32> for Scalar {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl From<Scalar> for i32 {
    fn from(value: Scalar) -> Self {
        value.0
    }
}

impl From<Tiles> for Scalar {
    fn from(value: Tiles) -> Self {
        Self(value.0)
    }
}

impl std::ops::Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Deref for Scalar {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tiles(i32);

impl From<i32> for Tiles {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::ops::AddAssign for Tiles {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl From<Scalar> for Tiles {
    fn from(value: Scalar) -> Self {
        Self(value.0)
    }
}

#[derive(Deserialize, Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ticks(i32);

impl From<i32> for Ticks {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl std::ops::Add for Ticks {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::SubAssign for Ticks {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl From<Ticks> for i32 {
    fn from(value: Ticks) -> Self {
        value.0
    }
}

/// # Errors
/// Returns an error if the given file cannot be found
pub fn read_file<T>(path: &str) -> Result<HashMap<String, T>>
where
    T: NamedData,
{
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str::<Vec<T>>(&data)?
        .into_iter()
        .map(|x| (x.get_name().to_owned(), x))
        .collect::<HashMap<_, _>>())
}
