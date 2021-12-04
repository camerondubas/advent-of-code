const output = (input) => {
  const groups = input
    .split("\n\n")
    .map((group) => group.split("\n").map((row) => row.split("")));

  let sum = 0;
  for (const group of groups) {
    let occurences = {};

    group.forEach((person) => {
      person.forEach(
        (response) =>
          (occurences[response] = occurences[response]
            ? occurences[response] + 1
            : 1)
      );
    });

    const val = Object.entries(occurences).reduce(
      (acc, cur) => acc + (cur[1] === group.length ? 1 : 0),
      0
    );

    sum += val;
  }

  return sum;
};

module.exports = output;
