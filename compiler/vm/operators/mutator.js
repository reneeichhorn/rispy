const { expressions } = require('../context');

module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  expressions(value)[config.type](
    config.expression,
    expressions(value)[config.value.type](config.value.expression)
  );
}
