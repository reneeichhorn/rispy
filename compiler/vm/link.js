const EventEmitter = require('events').EventEmitter;

class Linkable extends EventEmitter {
  constructor(value) {
    super();
    this.value = value;
  }

  getValue() { return this.value; }

  copy(value) {
    const copy = new Linkable(value);
    copy.on('used', (...args) => {
      this.emit('used', ...args);
      this.emit('linkedTo', ...args);
    });

    return copy;
  }
}

module.exports = Linkable;
