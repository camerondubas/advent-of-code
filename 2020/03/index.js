const input = require('./input');
const sanitizedInput = input.split('\n').filter(Boolean);

let treesEncountered = (input) => {
  let treesEncountered = 0;
  let column = 0;
  let row = 0;
  let rows = input.map(item => item.split(''))

  while (row < rows.length - 1) {
    row += 1
    column += 3;

    const currentRow = rows[row];
    const currentColumn = column % currentRow.length
    treesEncountered = currentRow[currentColumn] === '#' ? treesEncountered + 1 : treesEncountered;
  }

  return treesEncountered
}
console.log(treesEncountered(sanitizedInput))
