var addon = require('../native/lib');

const ERROR = 50;
const WARN = 40;
const INFO = 30;
const DEBUG = 20;
const TRACE = 10;

var instance;

function isStacktrace(arg) {
    if (!arg) {
        return false
    } else if (!arg.startsWith('Trace:')) {
        return false
    } else {
        const parts = arg.split(/\r?\n/);
        return parts.length >= 2 && parts.slice(1).every(val => val.trim().startsWith('at'));
    }
}

function apply(level) {
    return function(...args) {
        addon.log(instance, isStacktrace(args[0]) ? TRACE : level, args)
    }
}

module.exports.configure = function (api_key, options) {
    if (instance) throw "[dlog] configure(<API_KEY>) may only be called once"
    else if (typeof api_key !== 'string') throw "[dlog] Please provide a valid API_KEY"
    else instance = addon.configure(api_key, options || {})

    process.on('exit', addon.cleanUp.bind(null, instance));
    process.on('SIGINT', addon.cleanUp.bind(null, instance));

    const [error, warn, info, log, debug] = [
        console.error,
        console.warn,
        console.info,
        console.log,
        console.debug
    ];

    console.error = apply(ERROR);
    console.warn = apply(WARN);
    console.info = apply(INFO);
    console.log = apply(INFO);
    console.debug = apply(DEBUG);
}

module.exports.with_dlog = function (API_KEY, handler, options) {
    this.configure(API_KEY, options)
    return async function(...args) {
        const res = await Promise.resolve(handler(...args))
        addon.flush(instance)
        return res
    }
}