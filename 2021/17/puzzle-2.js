const output = (input) => {
  let [x1, x2, y1, y2] = [...input.matchAll(/[x|y]=(-?\d*)\.\.(-?\d*)/g)]
    .map((match) => [parseInt(match[1]), parseInt(match[2])])
    .flat();

  let count = 0;

  for (let x = 0; x <= x2; x++) {
    for (let y = y1; y <= 500; y++) {
      let isValid = buildPath([x, y], { x1, y1, x2, y2 });
      if (isValid) {
        count++;
      }
    }
  }

  return count;
};

let buildPath = (velocity, box) => {
  let point = [0, 0];

  let step = () => {
    point = [point[0] + velocity[0], point[1] + velocity[1]];

    let inXRange = point[0] >= box.x1 && point[0] <= box.x2;
    let inYRange = point[1] >= box.y1 && point[1] <= box.y2;
    let isTooLow = point[1] < box.y1;
    let noXVelocity = velocity[0] === 0;

    if (inXRange && inYRange) {
      return true;
    }

    if ((noXVelocity && !inXRange) || isTooLow) {
      return false;
    }

    if (velocity[0] !== 0) {
      velocity[0] += velocity[0] > 0 ? -1 : 1;
    }

    velocity[1] -= 1;

    return step();
  };

  return step();
};

module.exports = output;
