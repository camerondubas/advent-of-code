const input = require("./input");
// const input = `
// pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
// hcl:#623a2f

// eyr:2029 ecl:blu cid:129 byr:1989
// iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

// hcl:#888785
// hgt:164cm byr:2001 iyr:2015 cid:88
// pid:545766238 ecl:hzl
// eyr:2022

// iyr:2019
// hcl:#602927 eyr:1967 hgt:170cm
// ecl:grn pid:012533040 byr:1946

// hcl:dab227 iyr:2012
// ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

// iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
// `;
const sanitizedInput = input
  .split("\n\n")
  .map((item) => item.split(/ |\n+/g).filter(Boolean));

let validatePassports = (passports) => {
  let fields = {
    byr: input => {
      return input.length === 4 && input >= 1920 && input <= 2002
    },
    iyr: input => {
      return input.length === 4 && input >= 2010 && input <= 2020
    },
    eyr: input => {
      return input.length === 4 && input >= 2020 && input <= 2030
    },
    hgt: input => {
      if (input.includes('cm')) {
        let value = parseInt(input.split('cm')[0]);
        return value >= 150 && value <= 193
      }

      if (input.includes('in')) {
        let value = parseInt(input.split('in')[0]);
        return value >= 59 && value <= 76
      }

      return false
    },
    hcl: input => {
      return /#[0-9a-f]{6}/.test(input)
    },
    ecl: input => {
      return /((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)){1}/.test(input)
    },
    pid: input => {
      return /[0-9]{9}/.test(input)
    },
    cid: () => false
  };
  let a = passports.filter((passport) => {
    let count = passport.reduce(
      (prev, field) => (fields[field.split(":")[0]](field.split(":")[1]) ? prev + 1 : prev),
      0
    );
    return count === 7;
  }).length;

  return a;
};

console.log(validatePassports(sanitizedInput));
