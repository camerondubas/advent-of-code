const output = (input) => {
  return input
    .split("\n")
    .map((input) => parseInt(input))
    .reduce((acc, _, idx, arr) => {
      const window = [arr[idx], arr[idx + 1], arr[idx + 2]];
      const window2 = [arr[idx + 1], arr[idx + 2], arr[idx + 3]];
      const currentAvg = window.reduce(
        (acc, currentValue) => acc + (currentValue || 0),
        0
      );
      const nextAvg = window2.reduce(
        (acc, currentValue) => acc + (currentValue || 0),
        0
      );
      const isIncrease = currentAvg < nextAvg;
      return isIncrease ? acc + 1 : acc;
    }, 0);
};

module.exports = output;
