const pairs = {
  "(": ")",
  "[": "]",
  "{": "}",
  "<": ">",
};

const points = {
  ")": 1,
  "]": 2,
  "}": 3,
  ">": 4,
};

const output = (input) => {
  let lines = input.split("\n");
  let incompleteLines = [];

  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    let chunks = [];

    for (let idx = 0; idx < line.length; idx++) {
      let char = line[idx];

      if (pairs[char]) {
        chunks.unshift(pairs[char]);
      } else if (chunks[0] === char) {
        chunks.shift();
      } else {
        break;
      }

      if (idx === line.length - 1) {
        incompleteLines.push(chunks);
      }
    }
  }

  const sortedPoints = incompleteLines
    .map((chunk) => chunk.reduce((prev, cur) => prev * 5 + points[cur], 0))
    .sort((a, b) => a - b);

  const middleValue = sortedPoints[Math.floor(sortedPoints.length / 2)];

  return middleValue;
};

module.exports = output;
