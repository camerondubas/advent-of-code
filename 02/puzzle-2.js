const output = (input) => {
  let depth = 0;
  let horizontalPosition = 0;
  let aim = 0;

  for (const instruction of input.split("\n")) {
    const [direction, amount] = instruction.split(" ");
    const amountNumber = parseInt(amount);

    if (direction === "forward") {
      horizontalPosition += amountNumber;
      depth = depth + aim * amountNumber;
    } else if (direction === "up") {
      aim -= amountNumber;
    } else if (direction === "down") {
      aim += amountNumber;
    }
  }
  return depth * horizontalPosition;
};

module.exports = output;
