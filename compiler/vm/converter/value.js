const { expressions } = require('../context');

module.exports = (value, config, converters) => {
  const result = expressions(value)[config.value.type](config.value.value);
  return result;
};
