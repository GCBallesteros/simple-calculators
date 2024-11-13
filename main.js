import init, {
  calculate_twos_complement,
  decimal_to_twos_complement,
} from "./rust/pkg/rust.js";

init().then(() => {
  window.convertToDecimal = function () {
    const binaryInput = document.getElementById("binaryInput").value.trim();
    const decimalOutputElement = document.getElementById("decimalOutput");
    decimalOutputElement.textContent = calculate_twos_complement(binaryInput);
  };

  window.convertToTwosComplement = function () {
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

    binaryOutputElement.textContent = decimal_to_twos_complement(
      decimalInput,
      sizeInput,
    );
  };
});
