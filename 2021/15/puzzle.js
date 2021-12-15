const getAdjacentPoints = (point, height, width) => {
  const { x, y } = point;

  let points = [];
  if (x > 0) {
    // Left
    points.push([x - 1, y]);
  }
  if (x < width - 1) {
    // Right
    points.push([x + 1, y]);
  }
  if (y > 0) {
    // Up
    points.push([x, y - 1]);
  }

  if (y < height - 1) {
    // Down
    points.push([x, y + 1]);
  }

  return points;
};

class Point {
  constructor(x, y, value, score, g, h, parent) {
    this.x = x;
    this.y = y;
    this.g = g;
    this.h = h;
    this.value = value;
    this.score = score;
    this.parent = parent;
  }
}

const buildPoint = ([x, y], grid, parent) => {
  const value = grid[y][x];
  let remainingY = grid.length - 1 - y;
  let remainingX = grid[0].length - 1 - x;
  let g = value + parent?.g || 0;
  let h = remainingX + remainingY;
  let score = g + h;

  return new Point(x, y, value, score, g, h, parent);
};

const output = (input) => {
  // Risk Level

  let grid = input
    .split("\n")
    .map((row) => row.split("").map((num) => parseInt(num, 10)));

  const width = grid[0].length;
  const height = grid.length;

  const startPoint = buildPoint([0, 0], grid);
  // const endPoint = new Point(10, 10);

  let currentPoint = startPoint;

  const openList = [startPoint];
  const closedList = [];

  // `Step

  const step = (num) => {
    // get lowest in openlist
    const lowestIndex = openList.reduce(
      (prev, cur, index) => {
        if (cur.score < prev[1]) {
          return [index, cur.score];
        }
        return prev;
      },
      [undefined, Infinity]
    )[0];

    currentPoint =
      lowestIndex !== undefined && openList.splice(lowestIndex, 1)[0];

    console.log("=============");
    console.log("Step: ", num);
    console.log("Current Point: ", currentPoint);
    if (currentPoint) {
      closedList.push(currentPoint);
    } else {
      throw "Fuck!";
    }

    // add posible spots to open list
    let adjacentPoints = getAdjacentPoints(currentPoint, height, width);
    adjacentPoints.forEach(([x, y]) => {
      let point = buildPoint([x, y], grid, currentPoint);
      if (
        closedList.find(
          (cPoint) => cPoint.x === point.x && cPoint.y === point.y
        )
      ) {
        return;
      }

      let indexInOpenList = openList.findIndex(
        (oPoint) => oPoint.x === point.x && oPoint.y === point.y
      );
      if (indexInOpenList > -1) {
        if (point.score < openList[indexInOpenList].score) {
          openList.splice(indexInOpenList, 1, point);
        }
      } else {
        openList.push(point);
        return;
      }
    });
  };

  // let steps = 1000;
  let index = 0;

  while (!(currentPoint.x === width - 1 && currentPoint.y === height - 1)) {
    step(index + 1);
    index++;
  }

  console.log("-------------");
  console.log("end");
  // console.log("Current Point: ", currentPoint);
  // console.log("Open List: ", openList);
  // console.log("Closed List: ", closedList.map(p));

  let sum = 0;
  let target = closedList.reverse()[0];
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
