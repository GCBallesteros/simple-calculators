import init, {
  calculate_twos_complement,
  decimal_to_twos_complement,
  lat_lon_to_xyz,
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
  window.convertLatLonToXYZ = function () {
    const latitude = parseFloat(
      document.getElementById("latitudeInput").value.trim(),
    );
    const longitude = parseFloat(
      document.getElementById("longitudeInput").value.trim(),
    );
    const height = parseFloat(
      document.getElementById("heightInput").value.trim(),
    );

    const xOutputElement = document.getElementById("xOutput");
    const yOutputElement = document.getElementById("yOutput");
    const zOutputElement = document.getElementById("zOutput");

    if (isNaN(latitude) || isNaN(longitude) || isNaN(height)) {
      xOutputElement.textContent = "Error: Invalid input.";
      yOutputElement.textContent = "Error: Invalid input.";
      zOutputElement.textContent = "Error: Invalid input.";
      return;
    }

    const [x, y, z] = lat_lon_to_xyz(latitude, longitude, height);

    // Display the results
    xOutputElement.textContent = x.toFixed(6);
    yOutputElement.textContent = y.toFixed(6);
    zOutputElement.textContent = z.toFixed(6);
  };
});
