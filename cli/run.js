const fs = require("fs");
const path = require("path");

const getPuzzle = async (year, day, isPartTwo, file) => {
  let filename = file || (isPartTwo ? "puzzle-2.js" : "puzzle.js");

  const puzzlePath = path.join(global.appRoot, year, day, filename);
  const { default: puzzle } = await import(puzzlePath);

  return puzzle;
};

const getInput = (year, day, isDummyInput) => {
  const filename = isDummyInput ? "input-dummy.txt" : "input.txt";

  return fs
    .readFileSync(path.join(global.appRoot, year, day, filename), "utf8")
    .trim();
};

const getRunner = (data) => {
  if (data.showMetrics) {
    return (puzzle, input) => {
      const puzzleName = data.file || (data.advanced ? "2" : "1");

      console.log("==================");
      console.log(
        `Puzzle: Day ${data.day}, ${data.file ? "File:" : "Part"} ${puzzleName}`
      );

      if (data.dummy) {
        console.log("Using dummy input");
      }
      console.log("------------------");
      let runtimes = [];
      let output;
      let t0b = performance.now();
      for (let index = 0; index < data.metricsRuns; index++) {
        let t0 = performance.now();
        output = puzzle(input);
        let t1 = performance.now();
        runtimes.push(t1 - t0);
      }
      let t1b = performance.now();

      const runtimesSum = runtimes.reduce((acc, cur) => acc + cur, 0);
      console.log("Number of Runs: ", data.metricsRuns);
      console.log(
        "Avg Runtime (ms):",
        (runtimesSum / runtimes.length).toFixed(4)
      );
      console.log("Slowest Run (ms):", Math.max(...runtimes).toFixed(4));
      console.log("Fastest Run (ms):", Math.min(...runtimes).toFixed(4));
      console.log("Total Runtime (ms):", (t1b - t0b).toFixed(4));
      console.log("Output: ", output);
      console.log("==================");
    };
  }
  return (puzzle, input) => puzzle(input);
};

const run = async (year, _day, options) => {
  const { dummy, advanced, metrics, metricsRuns, file } = options;
  const day = _day.padStart(2, "0");
  const data = {
    showMetrics: metrics,
    metricsRuns,
    day,
    advanced,
    dummy,
    file,
  };

  const input = getInput(year, day, dummy);
  const puzzle = await getPuzzle(year, day, advanced, file);
  const runner = getRunner(data);

  return runner(puzzle, input);
};

module.exports = run;
