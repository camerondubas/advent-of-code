const range = (start, end) => {
  const min = Math.min(start, end);
  const max = Math.max(start, end);
  return [...Array(max + 1 - min)].map((_, index) => min + index);
};

const getCoords = (line) => {
  let _coords = [];
  range(line.x1, line.x2).forEach((xCoord) => {
    range(line.y1, line.y2).forEach((yCoord) => {
      _coords.push(`${xCoord},${yCoord}`);
    });
  });
  return _coords;
};

const getDiagonalCoords = (line) => {
  const isXDec = line.x1 > line.x2;
  const isYDec = line.y1 > line.y2;
  let xRange = range(line.x1, line.x2);
  let yRange = range(line.y1, line.y2);

  xRange = isXDec ? xRange.reverse() : xRange;
  yRange = isYDec ? yRange.reverse() : yRange;
  return xRange.map((val, index) => `${val},${yRange[index]}`);
};

const output = (input) => {
  const counts = {};
  input
    .split("\n")
    .map((line) =>
      line
        .replace(" -> ", ",")
        .split(",")
        .map((coord) => parseInt(coord))
    )
    .map(([x1, y1, x2, y2]) => ({ x1, y1, x2, y2 }))
    .map((line) => {
      const isStraight = line.x1 === line.x2 || line.y1 === line.y2;
      const coords = isStraight ? getCoords(line) : getDiagonalCoords(line);
      coords.forEach((coord) => (counts[coord] = counts[coord] + 1 || 1));
    });

  const numDuplicates = Object.values(counts).filter(
    (coord) => coord > 1
  ).length;
  return numDuplicates;
};

module.exports = output;
