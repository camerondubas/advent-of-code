const output = (input) => {
  let rows = input.split("\n");

  let sum = 0;
  for (let idx = 0; idx < rows.length; idx++) {
    const row = rows[idx];
    for (let index = 0; index < row.length; index++) {
      const item = row[index];

      const left = index === 0 ? Infinity : row[index - 1];
      const right = index === row.length - 1 ? Infinity : row[index + 1];
      const up = idx === 0 ? Infinity : rows[idx - 1]?.[index];
      const down = idx === rows.length - 1 ? Infinity : rows[idx + 1]?.[index];

      // console.log("Item: ", item, "-> Sides: ", left, right, up, down);
      if (item < left && item < right && item < down && item < up) {
        // console.log(item);
        sum += 1 + parseInt(item);
      }
    }
  }

  return sum;
};

module.exports = output;
