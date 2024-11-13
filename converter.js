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
    const isNegative = binaryInput[0] === '1'; // Two's complement sign bit
    let decimalValue;

    if (isNegative) {
        // Invert the bits and convert to decimal
        const invertedBinary = binaryInput.split('').map(bit => bit === '0' ? '1' : '0').join('');
        decimalValue = -1 * (parseInt(invertedBinary, 2) + 1);
    } else {
        // Positive binary number
        decimalValue = parseInt(binaryInput, 2);
    }

    return decimalValue;
}

function convertToDecimal() {
    const binaryInput = document.getElementById('binaryInput').value.trim();
    const decimalOutputElement = document.getElementById('decimalOutput');

    if (!/^[01]+$/.test(binaryInput)) {
        decimalOutputElement.textContent = "Invalid input: Enter only 0s and 1s.";
        return;
    }

    decimalOutputElement.textContent = calculateTwosComplement(binaryInput);
}

