const output = (input) => {
  let [x1, x2, y1, y2] = [...input.matchAll(/[x|y]=(-?\d*)\.\.(-?\d*)/g)]
    .map((match) => [parseInt(match[1]), parseInt(match[2])])
    .flat();

  let velocity = [1, 1];
  let box = { x1, y1, x2, y2 };

  let path = buildPath(velocity, box);
  let steps = 0;
  let maxY = 0;

  while (steps < 500) {
    path = buildPath(velocity, box);

    if (path.valid) {
      if (path.maxY > maxY) {
        maxY = path.maxY;
      } else {
        break;
      }
    }

    velocity = path.adj
      ? velocity.map((coord, idx) => coord + path.adj[idx])
      : null;

    steps++;
  }

  return maxY;
};

let checkPoint = (coords, velocity, box) => {
  let isInXRange = coords[0] >= box.x1 && coords[0] <= box.x2;
  let isInYRange = coords[1] >= box.y1 && coords[1] <= box.y2;
  let isTooLow = coords[1] < box.y2;
  let noXVelocity = velocity[0] === 0;

  let path = {
    maxY: null,
    adj: "",
    valid: false,
    isComplete: false,
  };

  if (isInXRange && isInYRange) {
    // Good Path, increase height
    path.isComplete = true;
    path.valid = true;
    path.adj = [0, 1];
    return path;
  }

  if (noXVelocity) {
    if (coords[0] < box.x1) {
      // too left, increase x
      path.isComplete = true;
      path.adj = [1, 0];
      return path;
    } else if (coords[0] > box.x2) {
      path.isComplete = true;
      path.adj = [-1, 0];
      return path;
    }
  }

  if (isTooLow) {
    path.isComplete = true;
    path.adj = [0, 1];
    return path;
  }

  return path;
};

let buildPath = (_velocity, box) => {
  let points = [[0, 0]];
  let velocity = [..._velocity];

  let step = () => {
    let _postion = [...points[points.length - 1]];
    _postion[0] += velocity[0];
    _postion[1] += velocity[1];

    points.push(_postion);

    let path = checkPoint(_postion, velocity, box);

    if (path.isComplete) {
      path.maxY = Math.max(...points.map((point) => point[1]));
      path.velocity = _velocity;
      return path;
    }

    if (velocity[0] != 0) {
      velocity[0] += velocity[0] > 0 ? -1 : 1;
    }

    velocity[1] -= 1;

    return step();
  };

  return step();
};

module.exports = output;
