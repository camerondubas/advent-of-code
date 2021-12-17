const fs = require("fs");
const path = require("path");
const fse = require("fs-extra");
const fetch = require("node-fetch");

const srcDir = `path/to/file`;
const destDir = `path/to/destination/directory`;

// To copy a folder or file

const generate = async (year, day) => {
  console.log("GENERATE", year, day);
  let srcDir = path.join(global.appRoot, "__template__");
  let destDir = path.join(global.appRoot, year, day);
  fse.copySync(srcDir, destDir, { overwrite: false });

  let data = await fetch(`https://adventofcode.com/${year}/day/${day}/input`, {
    headers: {
      cookie: `session=${process.env.SESSION_ID}`,
    },
  });
  data = await data.text();
  fse.writeFileSync(path.join(destDir, "input.txt"), data, {
    overwrite: false,
  });
};

module.exports = generate;
