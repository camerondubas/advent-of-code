const output = (input) => {
  const uniqueLengths = [2, 3, 4, 7];
  return input
    .split("\n")
    .map((i) =>
      i
        .split("|")[1]
        .trim()
        .split(" ")
        .filter((i) => uniqueLengths.includes(i.length))
    )
    .flat().length;
};

module.exports = output;
