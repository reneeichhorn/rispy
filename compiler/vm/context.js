let context = {};

module.exports = {
  getContext() { return context; },
  setContext(val) { context = val; },
  expressions: (value) => ({
    accessor(expr) {
      console.log(`[Expression] Accessing ${expr} -> ${module.exports.getContext()[expr]}`);
      return module.exports.getContext()[expr];
    },
    mutator(expr, val) {
      console.log(`[Expression] Mutating ${expr} -> ${val.getValue()}`);
      const context = { ...module.exports.getContext(), [expr]: val };
      module.exports.setContext(context);
    },
    stream(expr) {
      console.log(`[Expression] Accessing stream.${expr} -> ${value.getValue()}`);
      if (expr === 'stream') {
        return value;
      }
      return value.getValue()[expr];
    },
    constant(expr) {
      return value.copy(expr);
    },
  }),
};
