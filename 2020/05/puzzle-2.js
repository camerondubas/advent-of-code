const binarySpacePartitioning = (items, lowerChar, upperChar) => {
  let range = [0, Math.pow(2, items.length) - 1];

  items.forEach((char) => {
    const diff = (range[1] - range[0]) / 2;
    if (char === lowerChar) {
      range[1] = Math.floor(range[1] - diff);
    }
    if (char === upperChar) {
      range[0] = Math.ceil(range[0] + diff);
    }
  });

  return range[0];
};

const output = (input) => {
  let seatIds = [];

  for (const item of input) {
    let rows = item.slice(0, 7).split("");
    let seats = item.slice(7, Infinity).split("");

    let row = binarySpacePartitioning(rows, "F", "B");
    let seat = binarySpacePartitioning(seats, "L", "R");

    let seatId = row * 8 + seat;
    seatIds.push(seatId);
  }

  const sorttedSeatIds = seatIds.sort((a, b) => a - b);
  let seatId;
  for (let index = 0; index < sorttedSeatIds.length; index++) {
    const potentialSeatId = sorttedSeatIds[index] + 1;
    const isMatch = sorttedSeatIds[index + 1] !== potentialSeatId;
    if (isMatch) {
      seatId = potentialSeatId;
      break;
    }
  }

  return seatId;
};

module.exports = output;
