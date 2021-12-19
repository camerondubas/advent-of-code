#!/usr/bin/env node
const { Command } = require("commander");
const path = require("path");
const { run, generate } = require("../cli");
require("dotenv").config();

const program = new Command();
global.appRoot = path.resolve(path.join(__dirname, ".."));

program
  .version("2021.0.1")
  .argument("<year>", "Which year to run")
  .argument("<day>", "Which day to run")
  .option("-a --advanced", "Which puzzle to run on that day (1 or 2)", false)
  .option("-d --dummy", "Use dummy input", false)
  .option("-m --metrics", "Log runtime & other metrics", false)
  .option(
    "--metrics-runs <amount>",
    "Number of times to run the puzzle, for metrics logging",
    1000
  )
  .option(
    "-f --file <name>",
    "Specify name of file within the target day's directory"
  )
  .action((year, day, options) => {
    run(year, day, options)
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
  })
  .command("generate <year> <day>")
  .alias("g")
  .action((year, day, options) => {
    generate(year, day, options);
  });

program.parse();
