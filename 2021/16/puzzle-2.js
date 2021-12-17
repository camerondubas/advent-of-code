const output = (input) => {
  const program = new Program(input);
  program.run();
  return program.value;
};

module.exports = output;
class Program {
  constructor(hex) {
    this.#bits = this.#toBinary(hex);
    this.root = new Packet();
  }

  #bits;
  root;

  get value() {
    return this.root.value;
  }

  consume(num) {
    let output = this.#bits.substring(0, num);
    this.#bits = this.#bits.substring(num);
    return output;
  }

  run() {
    this.parse(this.root);
    return this;
  }

  parse(packet) {
    this.parseHeader(packet);

    if (packet.typeId === 4) {
      this.parseLiteral(packet);
    } else {
      this.parseOperator(packet);
    }
  }

  parseHeader(packet) {
    packet.version = this.#toDecimal(this.consume(3));
    packet.typeId = this.#toDecimal(this.consume(3));
  }

  parseLiteral(packet) {
    let bits = "";
    let isLastPart = false;

    while (!isLastPart) {
      isLastPart = this.consume(1) === "0";
      bits += this.consume(4);
    }

    packet.value = this.#toDecimal(bits);
  }

  parseOperator(packet) {
    packet.lengthTypeId = this.consume(1);

    let conditionBuilder;
    if (packet.lengthTypeId === "1") {
      conditionBuilder = () => {
        let numSubPackets = this.#toDecimal(this.consume(11));
        let index = 0;

        return () => {
          index++;
          return index <= numSubPackets;
        };
      };
    } else {
      conditionBuilder = () => {
        let packetLength = this.#toDecimal(this.consume(15));
        let initialBits = this.#bits.length;

        return () => initialBits - this.#bits.length < packetLength;
      };
    }

    const condition = conditionBuilder();

    while (condition()) {
      let _packet = new Packet();
      this.parse(_packet);
      packet.subPackets.push(_packet);
    }

    packet.value = this.#operations[packet.typeId](...packet.subPacketValues);
  }

  #toDecimal(bits) {
    return parseInt(bits, 2);
  }

  #toBinary(hex) {
    return hex
      .split("")
      .map((char) => parseInt(char, 16).toString(2).padStart(4, "0"))
      .join("");
  }

  #operations = {
    0: (...vals) => vals.reduce((acc, cur) => acc + cur, 0),
    1: (...vals) => vals.reduce((acc, cur) => acc * cur, 1),
    2: (...vals) => Math.min(...vals),
    3: (...vals) => Math.max(...vals),
    4: () => {},
    5: (a, b) => (a > b ? 1 : 0),
    6: (a, b) => (a < b ? 1 : 0),
    7: (a, b) => (a === b ? 1 : 0),
  };
}

class Packet {
  get subPacketValues() {
    return this.subPackets.map((subPacket) => subPacket.value);
  }

  lengthTypeId;
  typeId;
  version;
  value;
  subPackets = [];
}
