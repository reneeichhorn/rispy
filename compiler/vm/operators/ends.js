module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  if (isEnd) {
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
};
