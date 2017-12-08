const { setContext } = require('../context');

module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  if (config.object) {
    setContext(objects[config.object].create());
  }

  value.getValue()((innerValue, innerIsEnd) => {
    console.log(`[Execute] anonymous$ with ${innerValue.getValue()}`)

    config.outputs.forEach((innerOutput) => {
      operators[innerOutput.type](
        value.copy(innerValue.getValue()),
        innerOutput,
        innerIsEnd,
        streams,
        objects,
        operators,
        converters
      );
    });
  });
};
