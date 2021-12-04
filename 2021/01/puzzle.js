const output = (input) =>
  input
    .split("\n")
    .map((num) => parseInt(num))
    .reduce((acc, currentValue, idx, arr) => {
      return arr[idx - 1] < currentValue ? acc + 1 : acc;
    }, 0);

module.exports = output;
