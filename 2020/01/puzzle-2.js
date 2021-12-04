const fixExpenseReport = (input) => {
  const target = 2020;
  const sortedInput = input.sort((a, b) => a - b);
  const indices = [0, 1, 2];

  while (true) {
    const sum = indices.reduce((acc, index) => acc + sortedInput[index], 0);

    if (sum === target) {
      break;
    }

    if (indices[2] + 1 === input.length) {
      indices[0]++;
      indices[1] = indices[0] + 1;
      indices[2] = indices[0] + 1;
      continue;
    }

    if (sum < target) {
      indices[2]++;
    } else {
      indices[1]++;
      indices[2] = indices[1] + 1;
    }
  }

  return indices.reduce((acc, index) => acc * sortedInput[index], 1);
};

module.exports = (input) => {
  const sanitizedInput = input.split("\n").map((num) => parseInt(num));
  return fixExpenseReport(sanitizedInput);
};
