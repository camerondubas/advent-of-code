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
  const step = (edge, _pathSoFar) => {
    edge.forEach((value) => {
      let pathSoFar = [..._pathSoFar];

      if (
        (value === "start" && pathSoFar.length > 1) ||
        (value === value.toLowerCase() && pathSoFar.includes(value))
      ) {
        return;
      }

      pathSoFar.push(value);

      if (value === "end") {
        paths.push(pathSoFar);
        return;
      }

      step(graph.edges.get(value), pathSoFar);
    });
  };

  step(graph.edges.get("start"), ["start"]);

  console.log(
    "Paths",
    paths.map((p) => p.join()),
    paths.length
  );
  return paths;
};
const output = (input) => {
  let connections = input
    .split("\n")
    .map((connection) => connection.split("-"));

  let graph = buildGraph(connections);
  console.log(graph);
  let paths = buildPaths(graph);
  // console.log(graph.edges.get("start"));
  // console.log(paths);
};

module.exports = output;
