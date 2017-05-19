// $ npm install ffi
// $ node embed.js

var ffi = require('ffi');

var lib = ffi.Library('../target/release/libembed', {
    'process': ['void', []]
});

lib.process();

console.log("done!");
