module.exports = {
  root: true,
  parserOptions: {
    ecmaVersion: 2020,
    sourceType: "module",
  },
  plugins: ["prettier"],
  extends: ["eslint:recommended", "plugin:node/recommended"],
  rules: {
    "prettier/prettier": [
      "error",
      {
        endOfLine: "auto",
      },
    ],
    "node/no-unsupported-features": [
      "error",
      {
        ignores: ["dynamicImport"],
      },
    ],
  },
  env: {
    node: true,
  },
  globals: {
    import: true,
  },
};
