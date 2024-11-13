use wasm_bindgen::prelude::*;

// Export the functions to JavaScript
#[wasm_bindgen]
pub fn calculate_twos_complement(binary_input: &str) -> String {
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
pub fn decimal_to_twos_complement(decimal: i32, size: usize) -> String {
    if size == 0 {
        return "Error: Size must be greater than 0.".to_string();
    }

    let max_positive = (1 << (size - 1)) - 1;
    let min_negative = -(1 << (size - 1));

    if decimal > max_positive || decimal < min_negative {
        return "Error: Number does not fit in the specified size.".to_string();
    }

    if decimal >= 0 {
        format!("{:0>width$b}", decimal, width = size)
    } else {
        let binary = format!("{:0>width$b}", (-decimal), width = size);
        let ones_complement: String = binary
            .chars()
            .map(|bit| if bit == '0' { '1' } else { '0' })
            .collect();
        let result = i32::from_str_radix(&ones_complement, 2).unwrap() + 1;
        format!("{:0>width$b}", result, width = size)
    }
}

