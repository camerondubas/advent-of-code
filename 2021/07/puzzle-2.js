const sumOfSequence = (a, b) => {
  const n = Math.abs(a - b);
  return n * ((1 + n) / 2);
};
const output = (input) => {
  let crabPositions = input.split(",");
  let max = Math.max(...crabPositions);
  let min = Math.min(...crabPositions);

  let lowestCost = Infinity;
  for (let index = 1; index <= max - min; index++) {
    let cost = crabPositions
      .map((input) => sumOfSequence(input, index))
      .reduce((prev, cur) => prev + cur, 0);

    if (cost < lowestCost) {
      lowestCost = cost;
    }
  }

  return lowestCost;
};

module.exports = output;
