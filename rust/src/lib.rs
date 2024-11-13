use wasm_bindgen::prelude::*;
use thiserror::Error;

/// Custom error type for our application.
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
/// # Arguments
///
/// * `binary_input` - A binary string in two's complement format (e.g., "1101").
///
/// # Returns
///
/// A `Result<i32, TwosComplementError>` representing either the decimal value of the input binary string or an error.
///
/// # Examples
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
/// # Returns
///
/// A `Result<String, TwosComplementError>` containing the binary representation in two's complement format, or an error if invalid.
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

