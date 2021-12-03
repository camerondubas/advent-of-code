const getRating = (input, comparison) => {
  let instructions = input;

  for (let idx = 0; idx < input[0].length; idx++) {
    const bitCounter = instructions.reduce(
      (acc, cur) => acc + (cur[idx] === "1" ? 1 : -1),
      0
    );

    instructions = instructions.filter(
      (instruction) => instruction[idx] === (comparison(bitCounter) ? "1" : "0")
    );

    if (instructions.length === 1) {
      return parseInt(instructions[0], 2);
    }
  }
};

const output = (input) => {
  const oxygenGeneratorRating = getRating(input, (count) => count >= 0);
  const scrubberRating = getRating(input, (count) => count < 0);

  return oxygenGeneratorRating * scrubberRating;
};

module.exports = output;
