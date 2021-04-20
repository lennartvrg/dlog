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

function apply(original, level) {
    return function(...args) {
        original(...args)
        addon.log(instance, isStacktrace(args[0]) ? TRACE : level, args);
    }
}

module.exports.configure = function (api_key) {
    if (instance) throw "[dlog] configure(<API_KEY>) may only be called once"
    else if (typeof api_key !== 'string') throw "[dlog] Please provide a valid API_KEY"
    else instance = addon.configure(api_key)

    process.on('exit', addon.cleanUp.bind(null, instance));
    process.on('SIGINT', addon.cleanUp.bind(null, instance));

    const [error, warn, info, log, debug] = [
        console.error,
        console.warn,
        console.info,
        console.log,
        console.debug
    ];

    console.error = apply(error, ERROR);
    console.warn = apply(warn, WARN);
    console.info = apply(info, INFO);
    console.log = apply(log, INFO);
    console.debug = apply(debug, DEBUG);
}

module.exports.with_dlog = function (API_KEY, handler) {
    this.configure(API_KEY) 
    return async function(...args) {
        const res = await Promise.resolve(handler(...args))
        addon.flush(instance)
        return res
    }
}