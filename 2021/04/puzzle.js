const isMarked = (val) => val === "X";

const markNumber = (bingoNumber, card) => {
  for (let rowIndex = 0; rowIndex < card.length; rowIndex++) {
    const row = card[rowIndex];
    for (let colIndex = 0; colIndex < row.length; colIndex++) {
      const num = row[colIndex];
      if (num === bingoNumber) {
        card[rowIndex][colIndex] = "X";
        return;
      }
    }
  }
};

const hasBingo = (card) => {
  const gridSize = card.length;

  for (let index = 0; index < gridSize; index++) {
    const isRowBingo = card[index].every(isMarked);
    const isColBingo = card.every((row) => isMarked(row[index]));

    if (isRowBingo || isColBingo) {
      return true;
    }
  }
  return false;
};

const output = (input) => {
  const groups = input.split("\n\n");
  const bingoNumbers = groups.splice(0, 1)[0].split(",");
  const bingoCards = groups.map((card) =>
    card.split("\n").map((row) => row.match(/.{1,3}/g).map((num) => num.trim()))
  );

  for (const bingoNumber of bingoNumbers) {
    for (const bingoCard of bingoCards) {
      markNumber(bingoNumber, bingoCard);
      if (hasBingo(bingoCard)) {
        const sumOfUnmarked = bingoCard
          .flat()
          .reduce((acc, cur) => acc + (isMarked(cur) ? 0 : parseInt(cur)), 0);
        return sumOfUnmarked * bingoNumber;
      }
    }
  }
};

module.exports = output;
