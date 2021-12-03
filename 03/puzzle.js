const getRate = (input, comparison) => {
  let bitCounterArray = new Array(input[0].split("").length).fill(0);

  for (const instruction of input) {
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
  const gammaRate = getRate(input, (count) => count > 0);
  const epsilonRate = getRate(input, (count) => count < 0);

  return gammaRate * epsilonRate;
};

module.exports = output;
