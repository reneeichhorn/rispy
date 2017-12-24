const fs = require('fs');
const yaml = require('js-yaml');
const stdlib = require('./stdlib');
const operators = require('./operators');
const converters = require('./converter');
const Linkable = require('./link');

const program = yaml.load(fs.readFileSync(process.argv[2], 'utf8'));
const streams = {};

// Load std alias'
const streamAlias = {};
program.importFlows.forEach((imp) => {
  streamAlias[imp.from] = imp.name;
});

// Create objects
const objects = {};
Object.keys(program.namespace).forEach((objectName) => {
  const objectConfig = program.namespace[objectName];
  objects[objectName] = {
    create() {
      const initial = {};
      Object.keys(objectConfig.initialValue).forEach((key) => {
        initial[key] = new Linkable(objectConfig.initialValue[key]);
      });
      return initial;
    }
  };
});

// Create streams
Object.keys(program.flowDefinition).forEach((streamName) => {
  const self = streams[streamName];
  const selfConfig = program.flowDefinition[streamName];

  streams[streamName] = {
    validateIn(input) {
      // TODO: implement actual validation based on configuration
      return true;
    },

    validateOut(output) {
      // TODO: implement actual validation based on configuration
      return true;
    },

    execute(input, isEnd) {
      console.log(`[Execute] ${streamName} with`, input.getValue());

      input.emit('used', streamName, input, isEnd);

      selfConfig.outputs.forEach((outputConfig) => {
        operators[outputConfig.type](input, outputConfig, isEnd, streams, objects, operators, converters);
      });
    },
  };
});

// Load stdlib
stdlib((name, executeSelf, onExecute) => {
  // Streams:
  const alias = streamAlias[name];
  if (!alias) return;

  // build stream itself.
  const original = streams[alias];
  streams[alias] = {
    execute(input, isEnd) {
      console.log(`[Execute (STD)] ${alias} with`, input.getValue())
      //input.emit('used', alias, input, isEnd);
      onExecute(input, isEnd);

      if (original) original.execute(input, isEnd);
    }
  };

  // execute stream
  setTimeout(() => {
    const val = executeSelf();
    if (!val) { return }

    streams[alias].execute(val, true);
  });
}, (name, converter) => {
  // Converters:
  converters[name] = converter;
});
