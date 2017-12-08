module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  console.log('[IntoStream]', config.stream);
  // Execute links.
  if (config.links) {
    config.links.forEach((linkConfig) => {
      value.on('linkedTo', (streamName, linkedValue, linkEnd) => {
        console.log('link', linkConfig.to, streamName);
        if (linkConfig.to === streamName) {
          linkConfig.outputs.forEach((outputConfig) => {
            operators[outputConfig.type](
              linkedValue,
              outputConfig,
              linkEnd,
              streams,
              objects,
              operators,
              converters,
            );
          });
        }
      });
    });
  }

  if (config.converter) {
    // Execute target stream but convert it fist
    streams[config.stream]
      .execute(converters[config.converter.type](
        value,
        config.converter,
        converters,
      ));
  } else {
    // Execute target stream without converting.
    streams[config.stream]
      .execute(
        value,
        config.converter,
        converters,
      );
  }
};
