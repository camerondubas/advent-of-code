const step = (_counts, rules) => {
  let counts = {};
  let countsEntries = Object.entries(_counts);
  countsEntries.forEach(([pair, count]) => {
    let pair1 = rules[pair][0];
    let pair2 = rules[pair][1];
    counts[pair1] = counts[pair1] ? counts[pair1] + count : count;
    counts[pair2] = counts[pair2] ? counts[pair2] + count : count;
  });
  return counts;
};

const chunkEvery = (arr, size = 1, num = 1) => {
  let newArr = [];
  for (let index = 0; index < arr.length; index++) {
    if (index % num === 0) {
      let a = arr.slice(index, index + size);
      if (a.length === size) {
        newArr.push(a);
      }
    }
  }
  return newArr;
};

const createCounts = (str) => {
  let counts = {};
  chunkEvery(str, 2, 1).forEach(
    (pair) => (counts[pair] = counts[pair] ? counts[pair] + 1 : 1)
  );
  return counts;
};

const output = (input) => {
  let [template, rules] = input.split("\n\n");
  rules = rules
    .split("\n")
    .map((rule) => rule.split("->").map((part) => part.trim()))
    .reduce((acc, [pair, value]) => {
      acc[pair] = [pair[0] + value, value + pair[1]];
      return acc;
    }, {});

  const steps = 40;
  const lastLetter = template[template.length - 1];

  let counts = createCounts(template);

  for (let index = 0; index < steps; index++) {
    counts = step(counts, rules);
  }

  let letterCounts = Object.entries(counts).reduce(
    (acc, [pair, count]) => {
      acc[pair[0]] = acc[pair[0]] ? acc[pair[0]] + count : count;
      return acc;
    },
    { [lastLetter]: 1 }
  );

  let sortedCounts = Object.entries(letterCounts)
    .sort((a, b) => b[1] - a[1])
    .map((entry) => entry[1]);

  return sortedCounts[0] - sortedCounts[sortedCounts.length - 1];
};

module.exports = output;
