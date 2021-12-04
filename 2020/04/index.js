const input = require('./input');
// const input = `
// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
// byr:1937 iyr:2017 cid:147 hgt:183cm

// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
// hcl:#cfa07d byr:1929

// hcl:#ae17e1 iyr:2013
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm

// hcl:#cfa07d eyr:2025 pid:166559648
// iyr:2011 ecl:brn hgt:59in
// `;
const sanitizedInput = input
  .split("\n\n")
  .map((item) => item.split(/ |\n+/g).filter(Boolean));

let validatePassports = (passports) => {
  let fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
  return passports.filter((passport) => {
    let count = passport.reduce(
      (prev, field) => (fields.includes(field.split(":")[0]) ? prev + 1 : prev),
      0
    );

    return count === 7;
  }).length;
};
// let treesEncountered = (input) => {
//   let treesEncountered = 0;
//   let column = 0;
//   let row = 0;
//   let rows = input.map(item => item.split(''))

//   while (row < rows.length - 1) {
//     row += 1
//     column += 3;

//     const currentRow = rows[row];
//     const currentColumn = column % currentRow.length
//     treesEncountered = currentRow[currentColumn] === '#' ? treesEncountered + 1 : treesEncountered;
//   }

//   return treesEncountered
// }
console.log(validatePassports(sanitizedInput));
