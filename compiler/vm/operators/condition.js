const { getContext, setContext } = require('../context');

module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  let condition = false;

  const expressions = {
    accessor(expr) {
      console.log(`[Expression] Accessing ${expr} -> ${getContext()[expr]}`);
      return getContext()[expr];
    },
    mutator(expr, val) {
      console.log(`[Expression] Mutating ${expr} -> ${val.getValue()}`);
      const context = { ...getContext(), [expr]: val };
      setContext(context);
    },
    stream(expr) {
      console.log(`[Expression] Accessing stream.${expr} -> ${value.getValue()}`);
      if (expr === 'stream') {
        return value;
      }
      return value.getValue()[expr];
    },
  };

  switch (config.condition.type) {
    case 'lt':
      condition =
        expressions[config.condition.left.type](config.condition.left.expression).getValue() <
        expressions[config.condition.right.type](config.condition.right.expression).getValue();
    case 'gt':
      condition =
        expressions[config.condition.left.type](config.condition.left.expression).getValue() >
        expressions[config.condition.right.type](config.condition.right.expression).getValue();
      break;
  }

  if (condition) {
    if (config.reactions) {
      config.reactions.forEach(reaction => {
        expressions[reaction.type](
          reaction.expression,
          expressions[reaction.value.type](reaction.value.expression),
        );
      });
    }

    if (config.outputs) {
      config.outputs.forEach((innerOutput) => {
        operators[innerOutput.type](
          value,
          innerOutput,
          isEnd,
          streams,
          objects,
          operators,
          converters
        );
      });
    }
  }
};
