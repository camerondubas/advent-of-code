const range = (start, end) => {
  var min = Math.min(start, end);
  var max = Math.max(start, end);
  var range = [];

  for (let index = 0; index < max + 1 - min; index++) {
    range.push(min + index);
  }
  return range;
};

const output = (input) => {
  const coords = new Set();
  const duplicates = new Set();

  input.split("\n").map((line) => {
    const [x1, y1, x2, y2] = line.split(/\,| -> /).map((coord) => +coord);

    const xRange = range(x1, x2);
    const yRange = range(y1, y2);

    if (x1 === x2 || y1 === y2) {
      xRange.forEach((xCoord) => {
        yRange.forEach((yCoord) => {
          const coord = `${xCoord},${yCoord}`;
          if (coords.has(coord)) {
            duplicates.add(coord);
          } else {
            coords.add(coord);
          }
        });
      });
    } else {
      const isXDec = x1 > x2;
      const isYDec = y1 > y2;

      xRange.forEach((_, index) => {
        const coord = `${xRange[isXDec ? xRange.length - 1 - index : index]},${
          yRange[isYDec ? yRange.length - 1 - index : index]
        }`;
        if (coords.has(coord)) {
          duplicates.add(coord);
        } else {
          coords.add(coord);
        }
      });
    }
  });

  return duplicates.size;
};

module.exports = output;
