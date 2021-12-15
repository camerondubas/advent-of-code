const hasMultipleDuplicates = (chars) => {
  return new Set(chars).size < chars.length - 1;
};

class Graph {
  constructor() {
    this.edges = new Map();
  }

  nodes() {
    console.log("=== Nodes ===");
    let nodes = [];
    const loop = (node) => {
      nodes.push({
        value: node.value,
        connections: node.connections.map((child) => child.value),
      });
      node.connections.forEach((child) => {
        loop(child);
      });
    };

    loop(this.root);
    return nodes;
  }

  addConnection([a, b]) {
    if (!this.edges.has(a)) {
      this.edges.set(a, []);
    }

    this.edges.get(a).push(b);

    if (!this.edges.has(b)) {
      this.edges.set(b, []);
    }

    this.edges.get(b).push(a);
  }
}

const buildGraph = (connections) => {
  let graph = new Graph();
  connections.forEach((connection) => graph.addConnection(connection));
  return graph;
};

const buildPaths = (graph) => {
  let paths = [];
  const step = (edge, _pathSoFar, _smallCavesVisited) => {
    edge.forEach((value) => {
      let pathSoFar = [..._pathSoFar];
      let smallCavesVisited = [..._smallCavesVisited];

      if (value === "start" && pathSoFar.length > 1) {
        return;
      }

      if (value === value.toLowerCase()) {
        smallCavesVisited.push(value);
        if (hasMultipleDuplicates(smallCavesVisited)) {
          return;
        }
      }

      pathSoFar.push(value);

      if (value === "end") {
        paths.push(pathSoFar);
        return;
      }

      step(graph.edges.get(value), pathSoFar, smallCavesVisited);
    });
  };

  step(graph.edges.get("start"), ["start"], []);

  return paths;
};
const output = (input) => {
  let connections = input
    .split("\n")
    .map((connection) => connection.split("-"));

  let graph = buildGraph(connections);
  let paths = buildPaths(graph);

  return paths.length;
};

module.exports = output;
