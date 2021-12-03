const fs = require("fs");
const path = require("path");
const { Command } = require("commander");
const program = new Command();

program
  .version("2021.0.1")
  .argument("<day>", "Which day to run")
  .option("-a --advanced", "Which puzzle to run on that day (1 or 2)", false)
  .option("-d --dummy", "Use dummy input", false)
  .option("-m --metrics", "Log runtime & other metrics", false);

const runtime = (puzzle, input) => {
  console.time("Runtime");
  let output = puzzle(input);
  console.timeEnd("Runtime");
  return output;
};

const run = async (
  day,
  isAdvancedPuzzle = false,
  isDummyInput = false,
  trackRuntime = false
) => {
  const inputFilename = isDummyInput ? "input-dummy.txt" : "input.txt";
  const input = fs
    .readFileSync(path.join(__dirname, day, inputFilename), "utf8")
    .trim()
    .split("\n");

  const puzzleFilename = isAdvancedPuzzle ? "puzzle-2.js" : "puzzle.js";
  const puzzlePath = path.join(__dirname, day, puzzleFilename);

  const { default: puzzle } = await import(puzzlePath);

  let output = trackRuntime ? runtime(puzzle, input) : puzzle(input);
  return output;
};

program.parse();

const { dummy, advanced, metrics } = program.opts();
const day = program.args[0].padStart(2, "0");

run(day, advanced, dummy, metrics)
  .then((output) => {
    console.log(output);
  })
  .catch((err) => {
    if (err.code === "ENOENT") {
      console.log("Error: Could not find this puzzle.");
    } else {
      throw err;
    }
  });
