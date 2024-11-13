/**
 * Converts a two's complement binary string to its decimal value.
 *
 * @param {string} binaryInput - The binary string in two's complement format (e.g., "1101").
 * @returns {number} The decimal value of the input binary string.
 *
 * @example
 * // returns -3
 * calculateTwosComplement("1101");
 *
 * @example
 * // returns 5
 * calculateTwosComplement("0101");
 */
function calculateTwosComplement(binaryInput) {
  const isNegative = binaryInput[0] === "1"; // Two's complement sign bit
  let decimalValue;

  if (isNegative) {
    // Invert the bits and convert to decimal
    const invertedBinary = binaryInput
      .split("")
      .map((bit) => (bit === "0" ? "1" : "0"))
      .join("");
    decimalValue = -1 * (parseInt(invertedBinary, 2) + 1);
  } else {
    // Positive binary number
    decimalValue = parseInt(binaryInput, 2);
  }

  return decimalValue;
}

/**
 * Converts a decimal number to its two's complement binary representation.
 *
 * This function takes a signed decimal number and converts it to a two's
 * complement binary string representation with a specified bit size. If the
 * number cannot fit within the specified bit size, an error message is
 * returned. The two's complement representation allows negative numbers to be
 * encoded in binary and supports efficient arithmetic operations.
 *
 * @param {number} decimal - The signed decimal number to convert.
 * @param {number} size - The bit size of the two's complement representation.
 *                        Must be greater than 0.
 * @returns {string} - A string containing the binary representation of the
 *                     number in two's complement format. If the size is not
 *                     valid or the number does not fit within the specified size,
 *                     an error message is returned.
 *
 * @example
 * decimalToTwosComplement(5, 8);    // Returns "00000101"
 * decimalToTwosComplement(-5, 8);   // Returns "11111011"
 * decimalToTwosComplement(128, 8);  // Returns "Error: Number does not fit in the specified size."
 * decimalToTwosComplement(5, -1);   // Returns "Error: Size must be greater than 0."
 */
function decimalToTwosComplement(decimal, size) {
  if (size <= 0) {
    return "Error: Size must be greater than 0.";
  }

  const maxPositive = (1 << (size - 1)) - 1;
  const minNegative = -(1 << (size - 1));

  // Check if the number fits within the specified size
  if (decimal > maxPositive || decimal < minNegative) {
    return "Error: Number does not fit in the specified size.";
  }

  if (decimal >= 0) {
    let binary = decimal.toString(2);
    return binary.padStart(size, "0");
  } else {
    let binary = Math.abs(decimal).toString(2);
    let padded_binary = binary.padStart(size, "0");
    let onesComplement = padded_binary
      .split("")
      .map((bit) => (bit === "0" ? "1" : "0"))
      .join("");
    let result = (parseInt(onesComplement, 2) + 1).toString(2);
    return result;
  }
}

function convertToDecimal() {
  const binaryInput = document.getElementById("binaryInput").value.trim();
  const decimalOutputElement = document.getElementById("decimalOutput");

  if (!/^[01]+$/.test(binaryInput)) {
    decimalOutputElement.textContent = "Invalid input: Enter only 0s and 1s.";
    return;
  }

  decimalOutputElement.textContent = calculateTwosComplement(binaryInput);
}

function convertToTwosComplement() {
  const decimalInput = parseInt(
    document.getElementById("decimalInput").value.trim(),
    10,
  );
  const sizeInput = parseInt(
    document.getElementById("sizeInput").value.trim(),
    10,
  );
  const binaryOutputElement = document.getElementById("binaryOutput");

  if (isNaN(decimalInput) || isNaN(sizeInput) || sizeInput <= 0) {
    binaryOutputElement.textContent = "Error: Please enter valid values.";
    return;
  }

  const binaryRepresentation = decimalToTwosComplement(decimalInput, sizeInput);
  binaryOutputElement.textContent = binaryRepresentation;
}
