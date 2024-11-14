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
/// assert!((x - 6378137.0 - 1000.0).abs() < 1e-6);
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

//
// Find best UTM zone for a position
//

/// Custom error type for UTM zone calculation using `thiserror`
#[derive(Debug, Error)]
pub enum UTMZoneError {
    #[error("Invalid longitude: {0}. Longitude must be between -180 and 180 degrees.")]
    InvalidLongitude(f64),
    #[error("Invalid latitude: {0}. Latitude must be between -90 and 90 degrees.")]
    InvalidLatitude(f64),
    #[error("{0}")]
    CalculationError(String),
}

/// Returns the MGRS latitude band letter for a given latitude
fn get_mgrs_latitude_band(latitude: f64) -> Result<char, UTMZoneError> {
    if latitude < -80.0 || latitude >= 84.0 {
        return Err(UTMZoneError::InvalidLatitude(latitude));
    }

    let bands: Vec<char> = ('C'..='X')
        .filter(|&c| c != 'I' && c != 'O')
        .collect();

    let index = ((latitude + 80.0) /  8.0).floor() as usize;
    Ok(bands[index])
}

/// Calculates the UTM zone number and MGRS latitude band for a given latitude and longitude.
/// 
/// The function handles special UTM zone cases, such as areas in Norway and Svalbard, where
/// UTM zones deviate from the regular 6-degree longitudinal spacing. It also incorporates
/// the MGRS latitude bands, which range from 'C' to 'X' (excluding 'I' and 'O').
/// 
/// # Returns
/// - A `Result` containing the UTM zone number and MGRS latitude band, or an error if the inputs
///   are outside the valid latitude or longitude range.
/// 
/// # Examples
/// ```
/// use rust::calculate_utm_zone;
///
/// // General case in the Northern Hemisphere
/// let (zone, band) = calculate_utm_zone(40.0, -75.0).unwrap();
/// assert_eq!(zone, 18);
/// assert_eq!(band, 'T');
///
/// // General case in the Southern Hemisphere
/// let (zone, band) = calculate_utm_zone(-33.0, 151.0).unwrap();
/// assert_eq!(zone, 56);
/// assert_eq!(band, 'H');
///
/// // Special case in Norway
/// let (zone, band) = calculate_utm_zone(60.0, 5.0).unwrap();
/// assert_eq!(zone, 32);
/// assert_eq!(band, 'V');
///
/// // Special case in Svalbard
/// let (zone, band) = calculate_utm_zone(72.0, 7.0).unwrap();
/// assert_eq!(zone, 31);
/// assert_eq!(band, 'X');
///
/// // Near the equator
/// let (zone, band) = calculate_utm_zone(57.0, 1.0).unwrap();
/// assert_eq!(zone, 31);
/// assert_eq!(band, 'V');
///
/// // Another weird case between UK and Norway
/// let (zone, band) = calculate_utm_zone(0.0, 33.0).unwrap();
/// assert_eq!(zone, 36);
/// assert_eq!(band, 'N');
///
/// // Edge case at the boundary of valid latitude for MGRS
/// assert!(calculate_utm_zone(84.0, 15.0).is_err());
/// 
/// // Error case: Latitude out of range
/// assert!(calculate_utm_zone(90.1, 0.0).is_err());
/// 
/// // Error case: Longitude out of range
/// assert!(calculate_utm_zone(0.0, 181.0).is_err());
/// ```
pub fn calculate_utm_zone(latitude: f64, longitude: f64) -> Result<(u32, char), UTMZoneError> {
    if latitude < -90.0 || latitude > 90.0 {
        return Err(UTMZoneError::InvalidLatitude(latitude));
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err(UTMZoneError::InvalidLongitude(longitude));
    }

    let zone_number = if latitude > 55.0 && latitude < 64.0 && longitude > 2.0 && longitude < 6.0 {
        32
    } else if latitude > 71.0 && longitude >= 6.0 && longitude < 9.0 {
        31
    } else if latitude > 71.0 && (longitude >= 9.0 && longitude < 12.0 || longitude >= 18.0 && longitude < 21.0) {
        33
    } else if latitude > 71.0 && (longitude >= 21.0 && longitude < 24.0 || longitude >= 30.0 && longitude < 33.0) {
        35
    } else {
        ((longitude + 180.0) / 6.0).floor() as u32 % 60 + 1
    };


    let latitude_band = get_mgrs_latitude_band(latitude)?;

    Ok((zone_number, latitude_band))
}

#[wasm_bindgen]
pub fn get_utm_zone_from_lat_lon(latitude: f64, longitude: f64) -> Result<JsValue, JsValue> {
    match calculate_utm_zone(latitude, longitude) {
        Ok((zone_number, latitude_band)) => {
            let result = format!("{}{}", zone_number,  latitude_band);
            Ok(JsValue::from_str(&result))
        },
        Err(err) => Err(JsValue::from_str(&err.to_string())),
    }
}


