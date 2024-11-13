use wasm_bindgen::prelude::*;

/// Converts a two's complement binary string to its decimal value.
///
/// # Arguments
///
/// * `binary_input` - A binary string in two's complement format (e.g., "1101").
///
/// # Returns
///
/// A `String` representing the decimal value of the input binary string, or an error message
/// if the input is invalid.
///
/// # Examples
///
/// ```
/// use rust::calculate_twos_complement_rust;
///
/// assert_eq!(calculate_twos_complement_rust("1101"), "-3");
/// assert_eq!(calculate_twos_complement_rust("0101"), "5");
/// assert_eq!(calculate_twos_complement_rust("111"), "-1");
/// assert_eq!(calculate_twos_complement_rust("invalid"), "Invalid input: Enter only 0s and 1s.");
/// ```
pub fn calculate_twos_complement_rust(binary_input: &str) -> String {
    if binary_input.is_empty() || !binary_input.chars().all(|c| c == '0' || c == '1') {
        return "Invalid input: Enter only 0s and 1s.".to_string();
    }

    let is_negative = binary_input.starts_with('1');
    let decimal_value: i32;

    if is_negative {
        // Invert the bits and convert to decimal
        let inverted_binary: String = binary_input
            .chars()
            .map(|bit| if bit == '0' { '1' } else { '0' })
            .collect();
        decimal_value = -1 * (i32::from_str_radix(&inverted_binary, 2).unwrap() + 1);
    } else {
        // Positive binary number
        decimal_value = i32::from_str_radix(binary_input, 2).unwrap();
    }

    decimal_value.to_string()
}

#[wasm_bindgen]
pub fn calculate_twos_complement(binary_input: &str) -> String {
    calculate_twos_complement_rust(binary_input)
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
/// A `String` containing the binary representation in two's complement format, or an error
/// message if the size is invalid or the number does not fit.
///
/// # Examples
///
/// ```
/// use rust::decimal_to_twos_complement_rust;
///
/// assert_eq!(decimal_to_twos_complement_rust(5, 8), Ok("00000101".to_string()));
/// assert_eq!(decimal_to_twos_complement_rust(-5, 8), Ok("11111011".to_string()));
/// assert_eq!(
///     decimal_to_twos_complement_rust(128, 8),
///     Err("Error: Number does not fit in the specified size.".to_string())
/// );
/// assert_eq!(
///     decimal_to_twos_complement_rust(5, 0),
///     Err("Error: Size must be greater than 0.".to_string())
/// );
/// ```
pub fn decimal_to_twos_complement_rust(decimal: i32, size: usize) -> Result<String, String> {
    if size <= 0 {
        return Err("Error: Size must be greater than 0.".to_string());
    }

    let max_positive = (1 << (size - 1)) - 1;
    let min_negative = -(1 << (size - 1));

    if decimal > max_positive || decimal < min_negative {
        return Err("Error: Number does not fit in the specified size.".to_string());
    }

    if decimal >= 0 {
        Ok(format!("{:0>width$b}", decimal, width = size))
    } else {
        let binary = format!("{:0>width$b}", (-decimal), width = size);
        let ones_complement: String = binary
            .chars()
            .map(|bit| if bit == '0' { '1' } else { '0' })
            .collect();
        let result = i32::from_str_radix(&ones_complement, 2).unwrap() + 1;
        Ok(format!("{:0>width$b}", result, width = size))
    }
}

#[wasm_bindgen]
pub fn decimal_to_twos_complement(decimal: i32, size: usize) -> String {
    match decimal_to_twos_complement_rust(decimal, size) {
        Ok(result) => result,
        Err(e) => e,
    }
}
