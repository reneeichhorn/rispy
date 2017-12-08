let context = {};

module.exports = {
  getContext() { return context; },
  setContext(val) { context = val; },
};
