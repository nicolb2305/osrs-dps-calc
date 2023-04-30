use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

pub const SECONDS_PER_TICK: f64 = 0.6;

pub trait NamedData: for<'a> Deserialize<'a> {
    fn get_name(&self) -> &str;
}

#[derive(Deserialize, Debug, Clone, Copy, Default)]
pub struct Fraction {
    pub dividend: i32,
    pub divisor: i32,
}

impl Fraction {
    pub fn new(dividend: i32, divisor: i32) -> Self {
        Self { dividend, divisor }
    }
}

impl std::ops::Mul<Scalar> for Fraction {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar((self.dividend * rhs.0) / self.divisor)
    }
}

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Default,
    derive_more::From,
    derive_more::Add,
    derive_more::Sum,
)]
pub struct Percentage(i32);

impl std::ops::Mul<Scalar> for Percentage {
    type Output = Scalar;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Scalar(((100 + self.0) * rhs.0) / 100)
    }
}

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Mul,
    derive_more::Div,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::From,
    derive_more::Deref,
    derive_more::Sum,
)]
#[mul(forward)]
#[div(forward)]
pub struct Scalar(i32);

impl Scalar {
    pub fn new(value: i32) -> Self {
        Self(value)
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

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::From,
    derive_more::AddAssign,
)]
#[from(forward)]
pub struct Tiles(i32);

#[derive(
    Deserialize,
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::From,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
)]
pub struct Ticks(i32);

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
