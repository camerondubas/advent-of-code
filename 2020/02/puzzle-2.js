const validPasswordCounter = (passwordList) => {
  return passwordList.filter((pass) => isValidPassword(pass)).length;
};

const isValidPassword = (passwordCondition) => {
  const { pos1, pos2, character, password } =
    parsePasswordCondition(passwordCondition);
  let matches = 0;
  matches = password[pos1 - 1] === character ? matches + 1 : matches;
  matches = password[pos2 - 1] === character ? matches + 1 : matches;

  return matches === 1;
};

const parsePasswordCondition = (passwordCondition) => {
  const [range, character, password] = passwordCondition
    .split(" ")
    .filter(Boolean);
  return {
    pos1: parseInt(range.split("-")[0]),
    pos2: parseInt(range.split("-")[1]),
    character: character[0],
    password,
  };
};

module.exports = (input) => {
  const sanitizedInput = input.split("\n").filter(Boolean);
  return validPasswordCounter(sanitizedInput);
};
