const { expressions } = require('../context');

module.exports = (value, config, converters) => {
  let result = {};
  let cancel = false;

  Object.keys(config.mapping).forEach((key) => {
    if (cancel) return;
    if (key === 'stream') {
      result = expressions(value)[config.mapping[key].type](config.mapping[key].expression);
      cancel = true;
      return;
    }
    result[key] = expressions(value)[config.mapping[key].type](config.mapping[key].expression);
  });
  return result;
};
