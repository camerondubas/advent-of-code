const output = (input) => {
  const groups = input.split("\n\n").map((group) => group.replaceAll("\n", ""));

  const uniquesPerGroup = groups.map((group) => [...new Set(group)].length);
  const sumOfUniques = uniquesPerGroup.reduce((acc, cur) => acc + cur, 0);

  return sumOfUniques;
};

module.exports = output;
