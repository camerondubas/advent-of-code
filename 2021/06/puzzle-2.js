const output = (input) => {
  let maturityQueue = Array.from({ length: 9 }).fill(0);
  input.split(",").forEach((fish) => {
    maturityQueue[fish] = maturityQueue[fish] + 1;
  });

  const days = 256;
  const lifespan = 6;

  for (let day = 0; day < days; day++) {
    const maturing = maturityQueue.shift();
    maturityQueue.push(maturing);
    maturityQueue[lifespan] += maturing;
  }
  const fishCount = maturityQueue.reduce((prev, cur) => prev + cur, 0);

  return fishCount;
};

module.exports = output;
