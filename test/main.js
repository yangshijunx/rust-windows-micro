const { hello, getMicrophoneSensitivity } = require('../native/index.node');
console.log(hello("mmm"))
try {
    console.log(getMicrophoneSensitivity())
} catch (error) {
    console.log(error)
}
