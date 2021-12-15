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

const step = (template, rules) => {
  let pairs = chunkEvery(template.split(""), 2, 1).map((pair) => pair.join(""));

  let newPairs = [];

  for (let index = 0; index < pairs.length; index++) {
    const pair = pairs[index];

    let newChar = rules[pair];
    newPairs.push(`${pair[0] + newChar}`);

    if (index === pairs.length - 1) {
      newPairs.push(`${pair[1]}`);
    }
  }

  return newPairs.join("");
};
const output = (input) => {
  let [template, rules] = input.split("\n\n");
  const steps = 10;

  rules = rules
    .split("\n")
    .map((rule) => rule.split("->").map((part) => part.trim()))
    .map((rule) => ({ [rule[0]]: rule[1] }));

  rules = Object.assign({}, ...rules);

  for (let index = 0; index < steps; index++) {
    template = step(template, rules);
  }

  let letterCounts = template.split("").reduce((acc, cur) => {
    acc[cur] = acc[cur] ? acc[cur] + 1 : 1;
    return acc;
  }, {});

  let sortedCounts = Object.entries(letterCounts)
    .sort((a, b) => b[1] - a[1])
    .map((entry) => entry[1]);

  return sortedCounts[0] - sortedCounts[sortedCounts.length - 1];
};

module.exports = output;
