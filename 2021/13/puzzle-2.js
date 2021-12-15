const fillChar = ".";

const foldUp = (grid, idx) => {
  let top = grid.slice(0, idx);
  let bottom = grid.slice(idx + 1, grid.length);

  // Add Missing Rows to bottom
  const diff = top.length - bottom.length;
  for (let index = 0; index < diff; index++) {
    bottom.push(new Array(bottom[0].length).fill(fillChar));
  }

  bottom = bottom.reverse();
  return mergeArrs(top, bottom);
};

const foldLeft = (grid, idx) => {
  let left = grid.map((row) => row.slice(0, idx));
  let right = grid.map((row) => row.slice(idx + 1, row.length).reverse());
  return mergeArrs(left, right);
};

let mergeArrs = (a, b) => {
  for (let y = 0; y < a.length; y++) {
    const row = a[y];
    for (let x = 0; x < row.length; x++) {
      if (a[y][x] === "#" || b[y][x] === "#") {
        a[y][x] = "#";
      }
    }
  }

  return a;
};

const applyFold = (fold, grid) => {
  if (fold[0] === "x") {
    return foldLeft(grid, parseInt(fold[1]));
  } else if (fold[0] === "y") {
    return foldUp(grid, parseInt(fold[1]));
  }
};
const output = (input) => {
  let [points, folds] = input.split("\n\n");
  points = points.split("\n");
  points = points.map((point) =>
    point.split(",").map((num) => parseInt(num, 10))
  );

  let xMax = Math.max(...points.map(([x]) => x));
  let yMax = Math.max(...points.map(([_, y]) => y));

  let row = new Array(xMax + 1).fill(fillChar);
  let cols = new Array(yMax + 1).fill(fillChar);
  let grid = cols.map(() => [...row]);
  folds = folds.split("\n").map((fold) => fold.split(" ")[2].split("="));

  points.forEach(([x, y]) => {
    grid[y][x] = "#";
  });

  let output = grid;
  // folds.forEach((fold) => {
  //   output = applyFold(fold, output);
  // });

  output = output.map((row) => row.join("")).join("\n");
  fs = require("fs");
  fs.writeFile("output.txt", output, "utf-8", () => {});
  return output;
};

module.exports = output;
