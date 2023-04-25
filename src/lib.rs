use std::collections::HashMap;

use generics::NamedData;

pub mod equipment;
mod generics;
pub mod prayers;
pub mod unit;
mod weapon_callbacks;

/// # Errors
/// Returns an error if the given file cannot be found
pub fn read_file<T>(path: &str) -> Result<HashMap<String, T>, Box<dyn std::error::Error>>
where
    T: NamedData,
{
    let data = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str::<Vec<T>>(&data)?
        .into_iter()
        .map(|x| (x.get_name().to_owned(), x))
        .collect::<HashMap<_, _>>())
}
