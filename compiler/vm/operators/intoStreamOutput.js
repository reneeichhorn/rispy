module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  console.log('[IntoStreamOutput]', config.stream);
  value.emit('used', config.stream, value, isEnd);
}
