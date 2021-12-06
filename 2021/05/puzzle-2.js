const range = (start, end) => {
  const min = Math.min(start, end);
  const max = Math.max(start, end);
  return [...Array(max + 1 - min)].map((_, index) => min + index);
};

const getCoords = ([x1, y1, x2, y2]) => {
  let _coords = [];
  range(x1, x2).forEach((xCoord) => {
    range(y1, y2).forEach((yCoord) => {
      _coords.push(`${xCoord},${yCoord}`);
    });
  });
  return _coords;
};

const getDiagonalCoords = ([x1, y1, x2, y2]) => {
  const isXDec = x1 > x2;
  const isYDec = y1 > y2;
  let xRange = range(x1, x2);
  let yRange = range(y1, y2);

  xRange = isXDec ? xRange.reverse() : xRange;
  yRange = isYDec ? yRange.reverse() : yRange;
  return xRange.map((val, index) => `${val},${yRange[index]}`);
};

const output = (input) => {
  const counts = {};
  let duplicates = new Set();
  input.split("\n").map((line) => {
    let [x1, y1, x2, y2] = line
      .split(/\,| -> /)
      .map((coord) => parseInt(coord));

    const isStraight = x1 === x2 || y1 === y2;
    const coords = isStraight
      ? getCoords([x1, y1, x2, y2])
      : getDiagonalCoords([x1, y1, x2, y2]);
    coords.forEach((coord) => {
      if (counts[coord]) {
        duplicates.add(coord);
      } else {
        counts[coord] = true;
      }
    });
  });

  return duplicates.size;
};

module.exports = output;
