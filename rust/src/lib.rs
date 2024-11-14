use wasm_bindgen::prelude::*;
use thiserror::Error;
use std::f64::consts::PI;

#[derive(Error, Debug, PartialEq)]
pub enum TwosComplementError {
    #[error("Invalid input: Enter only 0s and 1s.")]
    InvalidInput,
    #[error("Error parsing binary input: {0}")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("Error: Size must be greater than 0.")]
    InvalidSize,
    #[error("Error: Number does not fit in the specified size.")]
    OverflowError,
}

/// Converts a two's complement binary string to its decimal value.
///
/// ```
/// use rust::calculate_twos_complement_rust;
/// use rust::TwosComplementError;
///
/// assert_eq!(calculate_twos_complement_rust("1101"), Ok(-3));
/// assert_eq!(calculate_twos_complement_rust("0101"), Ok(5));
/// assert_eq!(calculate_twos_complement_rust("111"), Ok(-1));
/// assert_eq!(
///     calculate_twos_complement_rust("invalid"),
///     Err(TwosComplementError::InvalidInput)
/// );
/// ```
pub fn calculate_twos_complement_rust(binary_input: &str) -> Result<i32, TwosComplementError> {
    if binary_input.is_empty() || !binary_input.chars().all(|c| c == '0' || c == '1') {
        return Err(TwosComplementError::InvalidInput);
    }

    let is_negative = binary_input.starts_with('1');
    let decimal_value: i32;

    if is_negative {
        // Invert the bits and convert to decimal
        let inverted_binary: String = binary_input
            .chars()
            .map(|bit| if bit == '0' { '1' } else { '0' })
            .collect();
        decimal_value = i32::from_str_radix(&inverted_binary, 2)
            .map(|val| -(val + 1)) // If parsing is successful, negate and add 1
            .map_err(TwosComplementError::ParseError)?;
    } else {
        // Positive binary number, parse normally
        decimal_value = i32::from_str_radix(binary_input, 2)
            .map_err(TwosComplementError::ParseError)?;
    }

    Ok(decimal_value)
}

#[wasm_bindgen]
pub fn calculate_twos_complement(binary_input: &str) -> String {
    match calculate_twos_complement_rust(binary_input) {
        Ok(result) => result.to_string(),
        Err(e) => e.to_string(),
    }
}

/// Converts a decimal number to its two's complement binary representation of a given bit size.
///
/// # Arguments
///
/// * `decimal` - The signed decimal number to convert.
/// * `size` - The bit size of the binary representation (must be greater than 0).
///
///
/// # Examples
///
/// ```
/// use rust::decimal_to_twos_complement_rust;
/// use rust::TwosComplementError;
///
/// assert_eq!(decimal_to_twos_complement_rust(5, 8), Ok("00000101".to_string()));
/// assert_eq!(decimal_to_twos_complement_rust(-5, 8), Ok("11111011".to_string()));
/// assert_eq!(
///     decimal_to_twos_complement_rust(128, 8),
///     Err(TwosComplementError::OverflowError)
/// );
/// assert_eq!(
///     decimal_to_twos_complement_rust(5, 0),
///     Err(TwosComplementError::InvalidSize)
/// );
/// ```
pub fn decimal_to_twos_complement_rust(decimal: i32, size: usize) -> Result<String, TwosComplementError> {
    if size <= 0 {
        return Err(TwosComplementError::InvalidSize);
    }

    let max_positive = (1 << (size - 1)) - 1;
    let min_negative = -(1 << (size - 1));

    if decimal > max_positive || decimal < min_negative {
        return Err(TwosComplementError::OverflowError);
    }

    if decimal >= 0 {
        Ok(format!("{:0>width$b}", decimal, width = size))
    } else {
        let binary = format!("{:0>width$b}", (-decimal), width = size);
        let ones_complement: String = binary
            .chars()
            .map(|bit| if bit == '0' { '1' } else { '0' })
            .collect();
        let result = i32::from_str_radix(&ones_complement, 2)
            .map_err(TwosComplementError::ParseError)?
            + 1;
        Ok(format!("{:0>width$b}", result, width = size))
    }
}

#[wasm_bindgen]
pub fn decimal_to_twos_complement(decimal: i32, size: usize) -> String {
    match decimal_to_twos_complement_rust(decimal, size) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    }
}


/// Converts latitude and longitude on the WGS84 ellipsoid to Cartesian XYZ coordinates.
/// 
/// # Parameters
/// - `latitude`: Latitude in degrees. Positive values indicate north of the equator, and negative values indicate south.
/// - `longitude`: Longitude in degrees. Positive values indicate east of the Prime Meridian, and negative values indicate west.
/// - `height`: Height above the WGS84 ellipsoid in meters. This is the elevation from the ellipsoid surface.
/// 
/// # Returns
/// A tuple `(X, Y, Z)` representing the Cartesian coordinates in meters.
/// 
/// # WGS84 Ellipsoid Constants
/// - `a`: Semi-major axis, 6378137.0 meters.
/// - `f`: Flattening factor, 1 / 298.257222101.
/// - `e2`: Square of eccentricity, calculated as `2 * f - f * f`.
/// 
/// # Example
/// ```
/// use rust::lat_lon_to_xyz_rust;
/// // Point on the equator at sea level
/// let (x, y, z) = lat_lon_to_xyz_rust(0.0, 0.0, 0.0);
/// assert!((x - 6378137.0).abs() < 1e-6);
/// assert!(y.abs() < 1e-6);
/// assert!(z.abs() < 1e-6);
///
/// // Point on the equator at the Prime Meridian with 1000 meters elevation
/// let (x, y, z) = lat_lon_to_xyz_rust(0.0, 0.0, 1000.0);
/// assert!((x - 6379137.0).abs() < 1e-6);
/// assert!(y.abs() < 1e-6);
/// assert!(z.abs() < 1e-6);
///
/// // Point on the equator at 90 degrees East
/// let (x, y, z) = lat_lon_to_xyz_rust(0.0, 90.0, 0.0);
/// assert!(x.abs() < 1e-6);
/// assert!((y - 6378137.0).abs() < 1e-6);
/// assert!(z.abs() < 1e-6);
/// ```
pub fn lat_lon_to_xyz_rust(latitude: f64, longitude: f64, height: f64) -> (f64, f64, f64) {
    // WGS84 ellipsoid constants
    let a = 6378137.0; // Semi-major axis in meters
    let f = 1.0 / 298.257222101; // Flattening
    let e2 = 2.0 * f - f * f; // Square of eccentricity

    // Convert latitude and longitude from degrees to radians
    let lat_rad = latitude * PI / 180.0;
    let lon_rad = longitude * PI / 180.0;

    // Calculate the prime vertical radius of curvature
    let n = a / (1.0 - e2 * lat_rad.sin().powi(2)).sqrt();

    // Calculate X, Y, and Z coordinates
    let x = (n + height) * lat_rad.cos() * lon_rad.cos();
    let y = (n + height) * lat_rad.cos() * lon_rad.sin();
    let z = (n * (1.0 - e2) + height) * lat_rad.sin();

    (x, y, z)
}

#[wasm_bindgen]
pub fn lat_lon_to_xyz(latitude: f64, longitude: f64, height: f64) ->  Vec<f64> {
    let (x, y, z) = lat_lon_to_xyz_rust(latitude, longitude, height);
    vec![x, y, z]
}
