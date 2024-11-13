function convertToDecimal() {
    const binaryInput = document.getElementById('binaryInput').value.trim();
    const decimalOutputElement = document.getElementById('decimalOutput');

    // Validate if the input is a binary number
    if (!/^[01]+$/.test(binaryInput)) {
        decimalOutputElement.textContent = "Invalid input: Enter only 0s and 1s.";
        return;
    }

    const length = binaryInput.length;
    const isNegative = binaryInput[0] === '1'; // Two's complement sign bit

    let decimalValue;

    if (isNegative) {
       let invertedBinary = binaryInput.split('').map(bit => bit === '0' ? '1' : '0').join('');
        decimalValue = -1 * (parseInt(invertedBinary, 2) + 1);
    } else {
        // Positive binary number
        decimalValue = parseInt(binaryInput, 2);
    }

    decimalOutputElement.textContent = decimalValue;
}

