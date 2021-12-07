const output = (input) => {
  let crabPositions = input.split(",");
  let max = Math.max(...crabPositions);
  let min = Math.min(...crabPositions);

  let count = max - min;
  let costs = [];
  for (let index = 0; index < count; index++) {
    let cost = crabPositions.map(
      (input) => Math.max(input, index + 1) - Math.min(input, index + 1)
    );
    costs.push(cost);
  }

  let sums = costs.map((cost) => cost.reduce((prev, cur) => prev + cur, 0));
  let lowestCost = Math.min(...sums);
  return lowestCost;
};

module.exports = output;
