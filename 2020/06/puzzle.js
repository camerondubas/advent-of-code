const output = (input) => {
  const groups = input.reduce(
    (acc, cur) => {
      if (cur !== "") {
        acc[acc.length - 1] += cur;
      } else {
        acc.push(cur);
      }
      return acc;
    },
    [""]
  );

  const uniquesPerGroup = groups.map((group) => [...new Set(group)].length);
  const sumOfUniques = uniquesPerGroup.reduce((acc, cur) => acc + cur, 0);

  return sumOfUniques;
};

module.exports = output;
