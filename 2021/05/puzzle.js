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
    .map((line) => ({ x1: line[0], y1: line[1], x2: line[2], y2: line[3] }))
    .filter((line) => line.x1 === line.x2 || line.y1 === line.y2)
    .map((line) =>
      getCoords(line).forEach(
        (coord) => (counts[coord] = counts[coord] + 1 || 1)
      )
    );

  const numDuplicates = Object.values(counts).filter(
    (coord) => coord > 1
  ).length;
  return numDuplicates;
};

module.exports = output;
