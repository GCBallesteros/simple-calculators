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
