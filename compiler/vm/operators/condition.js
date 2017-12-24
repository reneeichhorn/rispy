const { getContext, setContext, expressions } = require('../context');

module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  let condition = false;

  switch (config.condition.type) {
    case 'lt':
      condition =
        expressions(value)[config.condition.left.type](config.condition.left.expression).getValue() <
        expressions(value)[config.condition.right.type](config.condition.right.expression).getValue();
    case 'gt':
      condition =
        expressions(value)[config.condition.left.type](config.condition.left.expression).getValue() >
        expressions(value)[config.condition.right.type](config.condition.right.expression).getValue();
      break;
    case 'eq':
      condition =
        expressions(value)[config.condition.left.type](config.condition.left.expression).getValue() ==
        expressions(value)[config.condition.right.type](config.condition.right.expression).getValue();
      break;
  }

  if (condition) {
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
  } else {
    if (config.elseOutputs) {
      config.elseOutputs.forEach((innerOutput) => {
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
