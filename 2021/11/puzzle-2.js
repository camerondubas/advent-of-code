const size = 10;

const flash = (index, grid) => {
  if (grid[index] === 0) {
    return;
  }

  grid[index] = 0;

  const topOffsets = [-size - 1, -size, -size + 1];
  const bottomOffsets = [size - 1, size, size + 1];
  const leftOffsets = [-1, size - 1, -size - 1];
  const rightOffsets = [size + 1, 1, -size + 1];

  let offsets = [
    ...new Set([
      0,
      ...bottomOffsets,
      ...topOffsets,
      ...leftOffsets,
      ...rightOffsets,
    ]),
  ];

  const isTop = Math.floor(index / size) === 0;
  const isBottom = Math.floor(index / size) === size - 1;
  const isLeft = index % size === 0;
  const isRight = index % size === size - 1;

  offsets = offsets
    .filter((i) => (isLeft ? !leftOffsets.includes(i) : true))
    .filter((i) => (isRight ? !rightOffsets.includes(i) : true))
    .filter((i) => (isTop ? !topOffsets.includes(i) : true))
    .filter((i) => (isBottom ? !bottomOffsets.includes(i) : true));

  offsets.forEach((offset) => {
    let idx = index + offset;

    if (grid[idx] === 0) {
      return;
    }

    grid[idx] = grid[idx] + 1;

    if (grid[idx] > 9) {
      flash(idx, grid);
    }
  });
};

const output = (input) => {
  let octopuses = input
    .split("\n")
    .map((row) => row.split("").map((num) => parseInt(num, 10)))
    .flat();

  let step = 0;
  while (true) {
    step++;

    octopuses = octopuses.map((octopus) => octopus + 1);
    octopuses.forEach(
      (octopus, index) => octopus > 9 && flash(index, octopuses)
    );

    let flashesCount = octopuses.reduce((a, o) => (o === 0 ? a + 1 : a), 0);
    if (flashesCount === octopuses.length) {
      return step;
    }
  }
};

module.exports = output;
