const dlog = require('./../lib/index')

const handler = (event) => {
    console.log('This is my message for you: ' + JSON.stringify(event))
}

exports.handler =  dlog.with_dlog(process.env.DLOG_API_KEY, handler)

this.handler("Test")