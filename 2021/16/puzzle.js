let count = 0;

const consume = (num, arr) => {
  return arr.splice(0, num);
};

const binaryToDecimal = (arr) => {
  return parseInt(arr.join(""), 2);
};

const parseLiteral = (bits) => {
  let parts = [];
  while (bits[0] === "1") {
    consume(1, bits);
    parts.push(consume(4, bits));
  }
  consume(1, bits);
  parts.push(consume(4, bits));

  return parts.flat();
};

const parseOperator = (bits) => {
  let lengthTypeId = consume(1, bits);
  let lengthType = lengthTypeId[0] === "1" ? 11 : 15;
  console.log("Length Type Id", lengthTypeId);
  console.log("Length Type", lengthType);

  if (lengthType === 11) {
    let packetLength = 11;
    let numSubPackets = binaryToDecimal(consume(packetLength, bits));
    // let subPacket = consume(packetLength, bits);
    console.log("Parse N Packets");
    console.log("Num:", numSubPackets);
    console.log("Bits:", bits);
    for (let index = 0; index < numSubPackets; index++) {
      parse(bits);
    }
  } else {
    let packetLength = binaryToDecimal(consume(15, bits));
    let subPacket = consume(packetLength, bits);
    console.log("Packet Length", packetLength);
    parse(subPacket);
  }

  return bits;
};

const parse = (packet) => {
  console.log("=========");
  console.log("Parsing Packet: ", packet.join(""));

  // while (packet[0] === "0") {
  //   consume(1, packet);
  // }

  let version = binaryToDecimal(consume(3, packet));
  let typeId = binaryToDecimal(consume(3, packet));
  console.log("Version", version);
  console.log("Type Id", typeId);

  // Slow
  if (packet.every((bit) => bit === "0")) {
    return;
  }

  count += version;

  if (typeId !== 4) {
    console.log("-----");
    console.log("Parsing Operator");
    packet = parseOperator(packet);
  } else {
    console.log("-----");
    console.log("Parsing Literal");

    let value = binaryToDecimal(parseLiteral(packet));
    console.log("Value", value);
  }

  parse(packet);
};

const output = (input) => {
  console.log("Input:", input);
  let chars = input.split("");
  let packet = chars
    .map((char) => parseInt(char, 16).toString(2).padStart(4, "0"))
    .join("")
    .split("");

  parse(packet);
  return count;
};

module.exports = output;
