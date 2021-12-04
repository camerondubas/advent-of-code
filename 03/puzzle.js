const getRate = (instructions, comparison) => {
  let bitCounterArray = new Array(instructions[0].split("").length).fill(0);

  for (const instruction of instructions) {
    instruction.split("").forEach((bit, idx) => {
      bitCounterArray[idx] = bitCounterArray[idx] + (bit === "1" ? 1 : -1);
    });
  }

  return parseInt(
    bitCounterArray.map((bitCount) => (comparison(bitCount) ? 1 : 0)).join(""),
    2
  );
};

const output = (input) => {
  const instructions = input.split("\n");

  const gammaRate = getRate(instructions, (count) => count > 0);
  const epsilonRate = getRate(instructions, (count) => count < 0);

  return gammaRate * epsilonRate;
};

module.exports = output;
