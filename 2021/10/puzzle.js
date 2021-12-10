const pairs = {
  "(": ")",
  "[": "]",
  "{": "}",
  "<": ">",
};

const charPoints = {
  ")": 3,
  "]": 57,
  "}": 1197,
  ">": 25137,
};
const output = (input) => {
  let lines = input.split("\n");
  let chunks = [];
  let points = 0;

  for (let index = 0; index < lines.length; index++) {
    const line = lines[index];
    chunks = [];
    for (let idx = 0; idx < line.length; idx++) {
      let char = line[idx];
      if (pairs[char]) {
        chunks.unshift(pairs[char]);
      } else if (chunks[0] === char) {
        chunks.shift();
      } else {
        points += charPoints[char];
        break;
      }
    }
  }
  return points;
};

module.exports = output;
