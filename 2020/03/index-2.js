const input = require('./input');
const sanitizedInput = input.split('\n').filter(Boolean);

let treesEncountered = (input) => {
  const slopes = [[1,1], [3,1], [5,1], [7,1], [1, 2]]
  const results = slopes.map(slope => {
    let column = 0;
    let row = 0;
    let rows = input.map(item => item.split(''))

    let treesEncountered = 0;

    while (row < rows.length - 1) {
      row += slope[1]
      column += slope[0];

      const currentRow = rows[row];
      const currentColumn = column % currentRow.length
      treesEncountered = currentRow[currentColumn] === '#' ? treesEncountered + 1 : treesEncountered;
    }

    return treesEncountered
  });

  return results.reduce((prev, curr) => prev * curr, 1);
}
console.log(treesEncountered(sanitizedInput))
