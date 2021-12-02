const output = (input) => {
  let depth = 0;
  let horizontalPosition = 0;

  for (const instruction of input) {
    const [direction, amount] = instruction.split(" ");
    const amountNumber = parseInt(amount);

    if (direction === "forward") {
      horizontalPosition += amountNumber;
    } else if (direction === "up") {
      depth -= amountNumber;
    } else if (direction === "down") {
      depth += amountNumber;
    }
  }
  return depth * horizontalPosition;
};

module.exports = output;
