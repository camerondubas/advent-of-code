const output = (input) => {
  let fish = input.split(",");

  let day = 1;
  while (day <= 200) {
    let newFish = [];
    for (let index = 0; index < fish.length; index++) {
      const currentFish = fish[index];
      if (currentFish > 0) {
        fish[index] -= 1;
      } else {
        fish[index] = 6;
        newFish.push(8);
      }
    }

    fish = [...fish, ...newFish];
    day++;
  }

  return fish.length;
};

module.exports = output;
