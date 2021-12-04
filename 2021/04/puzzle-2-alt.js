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
    const isRowBingo = card[index].every((num) => num === "X");
    const isColBingo = card.every((row) => row[index] === "X");

    if (isRowBingo || isColBingo) {
      return true;
    }
  }
};

const output = (input) => {
  const groups = input.split("\n\n");
  const bingoNumbers = groups.splice(0, 1)[0].split(",");
  const bingoCards = groups.map((card) =>
    card.split("\n").map((row) => row.match(/.{1,3}/g).map((num) => num.trim()))
  );

  let winningCardCount = 0;

  let remainingCards = bingoCards;
  for (const bingoNumber of bingoNumbers) {
    for (const [index, bingoCard] of remainingCards.entries()) {
      if (!bingoCard) {
        continue;
      }
      markNumber(bingoNumber, bingoCard);

      if (hasBingo(bingoCard)) {
        delete remainingCards[index];

        winningCardCount++;
        if (winningCardCount === remainingCards.length) {
          const sumOfUnmarked = bingoCard
            .flat()
            .reduce((acc, cur) => acc + (cur !== "X" ? parseInt(cur) : 0), 0);
          return sumOfUnmarked * bingoNumber;
        }
      }
    }
  }
};

module.exports = output;
