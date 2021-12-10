const sortString = (str) => str.split("").sort().join("");

const findNumber = (patterns, condition) => {
  return sortString(
    patterns.splice(
      patterns.findIndex((p) => condition(p)),
      1
    )[0]
  );
};

const includesNumber = (input, target) =>
  input.split("").every((segment) => target.includes(segment));

const output = (input) => {
  let sum = 0;

  for (const entry of input.split("\n")) {
    let [patterns, outputDigits] = entry.split("|");
    patterns = patterns.trim().split(" ");
    outputDigits = outputDigits.trim().split(" ");

    const one = findNumber(patterns, (p) => p.length === 2);
    const four = findNumber(patterns, (p) => p.length === 4);
    const seven = findNumber(patterns, (p) => p.length === 3);
    const eight = findNumber(patterns, (p) => p.length === 7);
    const nine = findNumber(patterns, (p) => includesNumber(four, p));
    const three = findNumber(
      patterns,
      (p) => p.length === 5 && includesNumber(one, p)
    );

    const five = findNumber(
      patterns,
      (p) => p.length === 5 && includesNumber(p, nine)
    );
    const two = findNumber(patterns, (p) => p.length === 5);
    const zero = findNumber(patterns, (p) => includesNumber(one, p));
    const six = findNumber(patterns, () => true);

    let mapping = {
      [zero]: 0,
      [one]: 1,
      [two]: 2,
      [three]: 3,
      [four]: 4,
      [five]: 5,
      [six]: 6,
      [seven]: 7,
      [eight]: 8,
      [nine]: 9,
    };

    sum += parseInt(
      outputDigits.map((digit) => mapping[sortString(digit)]).join("")
    );
  }
  return sum;
};

module.exports = output;
