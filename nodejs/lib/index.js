var addon = require('../native/lib');

module.exports.configure = function (api_key) {
    var instance = addon.configure(api_key)

    process.on('exit', addon.cleanUp.bind(null, instance));
    process.on('SIGINT', addon.cleanUp.bind(null, instance));

    const [log, error] = [console.log, console.error];
    console.log = function(...args) {
        log(...args);
        addon.log(instance, args);
    };

    console.error = function(...args) {
        error(...args);
        addon.error(instance, args);
    };
}