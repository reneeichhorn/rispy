module.exports = (value, config, converters) => {
  return converters[config.stream](value, config, converters);
};
