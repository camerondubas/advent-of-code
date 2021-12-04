const input = require('./input');
const sanitizedInput = input.split('\n').map(num => parseInt(num));

const fixExpenseReport = (input) => {
  const target = 2020
  const sortedInput = input.sort((a,b) => b - a);
  for (const index in sortedInput) {
    const num1 = sortedInput[index];
    let num2;
    let sum;

    for (const _ in sortedInput) {
      num2 = sortedInput[sortedInput.length - 1]

      sum = num1 + num2;
      if (sum > target) {
        sortedInput.shift();
        break;
      } else if (sum < target) {
        sortedInput.pop();
      }
    }
    if (sum === target) {
      return num1 * num2;
    }
  }
}

const output = fixExpenseReport(sanitizedInput);
console.log(output);
