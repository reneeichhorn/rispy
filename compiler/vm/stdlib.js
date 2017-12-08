const Linkable = require('./link');

module.exports = (addStream, addOperator) => {
  /**
   * Application Start stream.
   */
  addStream('core/application-start', () => {
    return new Linkable({ applicationStart: true });
  }, () => {});

  /**
   * Stdout / Terminal output stream
   */
   addStream('core/terminal/stdout', () => {}, (value) => {
     process.stdout.write(value.getValue());
   });

   /**
    * Operator to convert a number into a string.
    */
   addOperator('core/converter/number/toString', (value) => {
     return value.copy(value.getValue().toString());
   });
};
