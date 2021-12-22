const output = (input) => {
  let snailNumbers = input.split("\n").map((n) => eval(n));
  const tree = new Tree(snailNumbers.shift());

  for (const snailNumber of snailNumbers) {
    tree.add(snailNumber);
    tree.reduce();
  }

  return tree.magnitude();
};

module.exports = output;

class Tree {
  constructor(data) {
    this.root = null;
    this.insert(data);
  }

  add(data) {
    if (!data) {
      return;
    }

    let currentData = this.toArray();
    this.root = null;

    this.insert([currentData, data]);
  }

  insert(data, parent = null) {
    let node = new Node(null, parent);

    if (!this.root) {
      this.root = node;
    }

    if (Array.isArray(data)) {
      node.left = this.insert(data[0], node);
      node.right = this.insert(data[1], node);
    } else {
      node.data = data;
    }

    return node;
  }
  traverse() {
    let output = [];

    let node = this.root;
    let _traverse = (node) => {
      if (node.left) {
        _traverse(node.left);
      }

      if (node.right) {
        _traverse(node.right);
      }
      if (node.data !== null) {
        output.push(node);
        return;
      }
    };

    _traverse(node);

    return output;
  }

  reduce() {
    let traversal = this.traverse();

    for (let index = 0; index < traversal.length; index++) {
      const node = traversal[index];

      if (node.depth > 4) {
        return this.explode(
          node.parent,
          traversal[index - 1],
          traversal[index + 2]
        );
      }
    }

    for (let index = 0; index < traversal.length; index++) {
      const node = traversal[index];

      if (node.data > 9) {
        return this.split(node);
      }
    }

    return this;
  }

  explode(node, left, right) {
    let leftVal = node.left.data;
    let rightVal = node.right.data;

    node.data = 0;
    node.left = null;
    node.right = null;

    if (left) {
      left.data += leftVal;
    }

    if (right) {
      right.data += rightVal;
    }

    return this.reduce();
  }

  split(node) {
    let leftVal = Math.floor(node.data / 2);
    let rightVal = Math.ceil(node.data / 2);

    node.data = null;
    node.left = this.insert(leftVal, node);
    node.right = this.insert(rightVal, node);

    return this.reduce();
  }

  magnitude(_node) {
    let node = _node || this.toArray();

    if (Array.isArray(node)) {
      if (Array.isArray(node[0])) {
        node[0] = this.magnitude(node[0]);
      }

      if (Array.isArray(node[1])) {
        node[1] = this.magnitude(node[1]);
      }

      if (!Array.isArray(node[0]) && !Array.isArray(node[1])) {
        return node[0] * 3 + node[1] * 2;
      }
    }
  }

  toArray(_node) {
    let node = _node || this.root;
    let arr = [null, null];

    if (node.left) {
      arr[0] = this.toArray(node.left);
    }
    if (node.right) {
      arr[1] = this.toArray(node.right);
    }

    if (node.data !== null) {
      arr = node.data;
    }

    return arr;
  }
}

class Node {
  constructor(data = null, parent = null) {
    this.data = data;
    this.parent = parent;
    this.left = null;
    this.right = null;
  }

  get depth() {
    return this.parent ? this.parent.depth + 1 : 0;
  }
}
