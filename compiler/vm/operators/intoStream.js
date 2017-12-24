module.exports = (value, config, isEnd, streams, objects, operators, converters) => {
  console.log('[IntoStream]', config.stream);
  // Execute links.
  if (config.links) {
    config.links.forEach((linkConfig) => {
      console.log('link establ,', linkConfig.to);
      const linkedListener = (streamName, linkedValue, linkEnd) => {
        if (linkEnd) {
          console.log('link ended', linkConfig.to);
          value.removeListener('linkedTo', linkedListener);
        }
        console.log('link checking', linkConfig.to, streamName);
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
      };

      value.on('linkedTo', linkedListener);
    });
  }

  if (config.converter) {
    // Execute target stream but convert it fist
    streams[config.stream]
      .execute(converters[config.converter.type](
        value,
        config.converter,
        converters,
      ), isEnd);
  } else {
    // Execute target stream without converting.
    streams[config.stream]
      .execute(
        value,
        isEnd
      );
  }
};
