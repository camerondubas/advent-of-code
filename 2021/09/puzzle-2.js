const getNeighborCoords = (x, y, rows) => {
  return [
    [x - 1, y],
    [x + 1, y],
    [x, y - 1],
    [x, y + 1],
  ].filter(
    ([x, y]) => x >= 0 && y >= 0 && x < rows[0].length && y < rows.length
  );
};

const output = (input) => {
  let rows = input
    .split("\n")
    .map((row) => row.split("").map((height) => parseInt(height, 10)));

  let basins = [];
  for (let y = 0; y < rows.length; y++) {
    const row = rows[y];
    for (let x = 0; x < row.length; x++) {
      const item = row[x];
      let coords = getNeighborCoords(x, y, rows);

      let isBasin = coords.every((coord) => item < rows[coord[1]][coord[0]]);
      if (isBasin) {
        basins.push([[x, y]]);
      }
    }
  }

  const shouldAddToBasin = (coords, currentItem, basin, rows) => {
    let item = rows[coords[1]]?.[coords[0]];
    if (!item || item === 9 || item < currentItem) {
      return false;
    }

    return !basin.find((item) => item.toString() === coords.toString());
  };

  let buildBasin = (_basin) => {
    let basin = [..._basin];
    for (let index = 0; index < basin.length; index++) {
      const [x, y] = basin[index];
      const value = rows[y][x];

      let neighborCoords = getNeighborCoords(x, y, rows);

      neighborCoords.forEach((coords) => {
        if (shouldAddToBasin(coords, value, basin, rows)) {
          basin.push(coords);
        }
      });
    }
    if (basin.length !== _basin.length) {
      return buildBasin(basin);
    } else {
      return basin;
    }
  };

  for (let index = 0; index < basins.length; index++) {
    const basin = basins[index];
    basins[index] = buildBasin(basin).length;
  }

  return basins
    .sort((a, b) => a - b)
    .reverse()
    .splice(0, 3)
    .reduce((prev, cur) => prev * cur, 1);
};

module.exports = output;
