module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  if (value.getValue() === null) return;
  console.log('[IntoStreamOutput]', config.stream);

  if (config.converter) {
    // Execute target stream but convert it first
    value.emit('used', config.stream, converters[config.converter.type](
        value, config.converter, converters,
    ), isEnd);
  } else {
    // Execute target stream without converting
    value.emit('used', config.stream, value, isEnd);
  }
}
