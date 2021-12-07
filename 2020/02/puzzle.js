const validPasswordCounter = (passwordList) => {
  return passwordList.filter((pass) => isValidPassword(pass)).length;
};

const isValidPassword = (passwordCondition) => {
  const { min, max, character, password } =
    parsePasswordCondition(passwordCondition);
  const occurences = password.match(new RegExp(character, "g"))?.length || 0;

  return occurences >= min && occurences <= max;
};

const parsePasswordCondition = (passwordCondition) => {
  const [range, character, password] = passwordCondition
    .split(" ")
    .filter(Boolean);
  return {
    min: parseInt(range.split("-")[0]),
    max: parseInt(range.split("-")[1]),
    character: character[0],
    password,
  };
};

module.exports = (input) => {
  const sanitizedInput = input.split("\n").filter(Boolean);
  return validPasswordCounter(sanitizedInput);
};
