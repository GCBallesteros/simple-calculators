import init, {
  calculate_twos_complement,
  decimal_to_twos_complement,
  lat_lon_to_xyz,
  get_utm_zone_from_lat_lon,
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
  window.convertLatLonToUTMZone = function () {
    const latitude = parseFloat(
      document.getElementById("latitudeForUTMInput").value.trim(),
    );
    const longitude = parseFloat(
      document.getElementById("longitudeForUTMInput").value.trim(),
    );
    const utmOutputElement = document.getElementById("utmOutput");

    // Validate inputs
    if (isNaN(latitude) || isNaN(longitude)) {
      utmOutputElement.textContent = "Error: Invalid input.";
      return;
    }

    try {
      // Call the WebAssembly function and get the result
      const utmZone = get_utm_zone_from_lat_lon(latitude, longitude);
      utmOutputElement.textContent = `UTM Zone: ${utmZone}`;
    } catch (e) {
      // Handle any errors from the Rust function
      utmOutputElement.textContent = `Error: ${e.message}`;
    }
  };
});
