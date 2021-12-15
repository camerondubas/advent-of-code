const getAdjacentPoints = (point, size) => {
  const { x, y } = point;

  let points = [];
  if (x > 0) {
    points.push([x - 1, y]);
  }
  if (x < size) {
    points.push([x + 1, y]);
  }
  if (y > 0) {
    points.push([x, y - 1]);
  }

  if (y < size) {
    points.push([x, y + 1]);
  }

  return points;
};

class Point {
  constructor(x, y, g, value, score, parent) {
    this.x = x;
    this.y = y;
    this.g = g;
    this.value = value;
    this.score = score;
    this.parent = parent;
    this.name = `${x},${y}`;
  }
}

const buildPoint = ([x, y], grid, parent) => {
  let tileSize = grid.length;
  let size = tileSize * 5;
  const xRisk = Math.floor(x / tileSize);
  const yRisk = Math.floor(y / tileSize);

  let baseValue = grid[y % tileSize][x % tileSize];
  let value = ((baseValue + xRisk + yRisk - 1) % 9) + 1;

  let remainingY = size - y;
  let remainingX = size - x;

  let g = value + parent?.g || 0;
  let h = remainingX + remainingY;
  let score = g + h;

  return new Point(x, y, g, value, score, parent, baseValue);
};

// This is an implmentation of an A* pathfinding algorithm
const output = (input) => {
  let grid = input
    .split("\n")
    .map((row) => row.split("").map((num) => parseInt(num, 10)));

  const size = grid.length * 5 - 1;
  const startPoint = buildPoint([0, 0], grid);
  let currentPoint = startPoint;
  const closedList = {};
  const openList = [startPoint];

  const step = (_) => {
    const lowestIndex = openList.reduce(
      (prev, cur, index) => {
        if (cur.score < prev[1]) {
          return [index, cur.score, cur.g];
        } else if (cur.score === prev[1]) {
          if (cur.g < prev[2]) {
            return [index, cur.score, cur.g];
          }
        }
        return prev;
      },
      [undefined, Infinity, Infinity]
    )[0];

    currentPoint =
      lowestIndex !== undefined && openList.splice(lowestIndex, 1)[0];
    closedList[currentPoint.name] = currentPoint;

    getAdjacentPoints(currentPoint, size).forEach(([x, y]) => {
      let point = buildPoint([x, y], grid, currentPoint);
      if (closedList[point.name]) {
        return;
      }

      let openListPoint = openList.find((oPoint) => oPoint.name === point.name);
      if (openListPoint) {
        if (point.score < openListPoint.score) {
          openListPoint.score = point.score;
          openListPoint.parent = point.parent;
        }
      } else {
        openList.push(point);
        return;
      }
    });
  };

  let index = 0;

  while (!(currentPoint.x === size && currentPoint.y === size)) {
    step(index + 1);
    index++;
  }

  let sum = 0;
  let target = currentPoint;
  while (true) {
    sum += target.value;

    let parent = target.parent;
    if (!parent) {
      break;
    }

    target = parent;
  }

  return sum - startPoint.value;
};

module.exports = output;
