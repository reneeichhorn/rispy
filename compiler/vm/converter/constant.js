const Linkable = require('../link');

module.exports = (value, config, converters) => {
  if (config.constantType === 'stream') {
    const stream = (execute) => config.value.forEach((valueConfig, i) => {
      execute(value.copy(valueConfig.value), config.value.length === (i+1));
    });
    return value.copy(stream);
  } else if (config.constantType === 'number') {
    return parseInt(value);
  } else if (config.constantType === 'string') {
    return value.toString();
  } else {
    console.error('Constant converter: Unknown constant type', config.constantType);
  }
};
