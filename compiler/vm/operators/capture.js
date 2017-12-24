let capturedValues = [];

module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  capturedValues.push(value);
    console.log('[Caputure] Added capture ->', value.getValue());

  if (isEnd) {
    const captured = [...capturedValues];
    console.log('[Caputure] Capture completed.', capturedValues.map(val => val.getValue()));
    capturedValues = [];
    const stream = (execute) => captured.forEach((val, i) => {
      execute(val, captured.length === (i+1));
    });

    config.outputs.forEach((innerOutput) => {
      operators[innerOutput.type](
        value.copy(stream),
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
