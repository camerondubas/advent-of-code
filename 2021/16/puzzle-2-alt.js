const binaryToDecimal = (arr) => {
  return parseInt(arr.join(""), 2);
};

const parseLiteral = (packet) => {
  let parts = [];
  while (packet.bits[0] === "1") {
    packet.consume(1);
    parts.push(packet.consume(4));
  }
  packet.consume(1);
  parts.push(packet.consume(4));

  return parts.flat();
};

const parseOperator = (packet) => {
  packet.lengthTypeId = packet.consume(1)[0];

  if (packet.lengthTypeId === "1") {
    let packetLength = 11;
    let numSubPackets = binaryToDecimal(packet.consume(packetLength));

    for (let index = 0; index < numSubPackets; index++) {
      let _packet = new Packet(packet.program);
      parse(_packet);
      packet.subPackets.push(_packet);
    }
  } else {
    let packetLength = binaryToDecimal(packet.consume(15));
    let initialBits = packet.bits.length;

    // TODO: Cleanup
    while (initialBits - packet.program.bits.length < packetLength) {
      let _packet = new Packet(packet.program);
      parse(_packet);
      packetBits = _packet.bits;
      packet.subPackets.push(_packet);
    }
  }

  packet.value = packet.operation();

  return packet;
};

const parse = (packet) => {
  packet.version = binaryToDecimal(packet.consume(3));
  packet.typeId = binaryToDecimal(packet.consume(3));

  if (packet.typeId !== 4) {
    packet = parseOperator(packet);
  } else {
    packet.value = binaryToDecimal(parseLiteral(packet));
  }
};

const output = (input) => {
  let chars = input.split("");
  let bits = chars
    .map((char) => parseInt(char, 16).toString(2).padStart(4, "0"))
    .join("")
    .split("");

  let program = new Program(bits);
  let packet = new Packet(program);

  parse(packet);

  return packet.value;
};

module.exports = output;
class Program {
  constructor(bits) {
    this.bits = [...bits];
  }

  consume(num) {
    return this.bits.splice(0, num);
  }

  operations = {
    0: (...vals) => vals.reduce((acc, cur) => acc + cur, 0),
    1: (...vals) => vals.reduce((acc, cur) => acc * cur, 1),
    2: (...vals) => Math.min(...vals),
    3: (...vals) => Math.max(...vals),
    4: () => {
      throw "Should not be here. RIP";
    },
    5: (a, b) => (a > b ? 1 : 0),
    6: (a, b) => (a < b ? 1 : 0),
    7: (a, b) => (a === b ? 1 : 0),
  };
}

class Packet {
  constructor(program) {
    this.program = program;
    this.bits = this.program.bits;
  }

  consume(num) {
    return this.program.consume(num);
  }

  operation() {
    let subPacketValues = this.subPackets.map((subPacket) => subPacket.value);
    return this.program.operations[this.typeId](...subPacketValues);
  }

  bits;
  lengthTypeId;
  typeId;
  version;
  value;
  subPackets = [];
}
