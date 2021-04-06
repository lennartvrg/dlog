var addon = require('../native');

console.log = (...args) => addon.log(args);

console.error = (...args) => addon.error(args);

module.exports.configure = addon.configure