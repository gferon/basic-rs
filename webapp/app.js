let wasm = {};

//
// BEGIN COPY-PASTE from wasm-bindgen generated code with import/export removed.
//
// To generate, first run and then copy the content from basic.js:
//
// ```
// cargo build --target wasm32-unknown-unknown --release
// wasm-bindgen target/wasm32-unknown-unknown/release/basic.wasm --out-dir \
//      webapp
// ```
//

const TextDecoder = typeof self === 'object' && self.TextDecoder
                        ? self.TextDecoder
                        : require('util').TextDecoder;

let cachedDecoder = new TextDecoder('utf-8');

let cachegetUint8Memory = null;
function getUint8Memory() {
  if (cachegetUint8Memory === null ||
      cachegetUint8Memory.buffer !== wasm.memory.buffer) {
    cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
  }
  return cachegetUint8Memory;
}

function getStringFromWasm(ptr, len) {
  return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
  if (cachedGlobalArgumentPtr === null) {
    cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
  }
  return cachedGlobalArgumentPtr;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
  if (cachegetUint32Memory === null ||
      cachegetUint32Memory.buffer !== wasm.memory.buffer) {
    cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
  }
  return cachegetUint32Memory;
}

const TextEncoder = typeof self === 'object' && self.TextEncoder
                        ? self.TextEncoder
                        : require('util').TextEncoder;

let cachedEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

  const buf = cachedEncoder.encode(arg);
  const ptr = wasm.__wbindgen_malloc(buf.length);
  getUint8Memory().set(buf, ptr);
  return [ ptr, buf.length ];
}
/**
 * @param {string} arg0
 * @returns {Output}
 */
function simple_execute(arg0) {
  const [ptr0, len0] = passStringToWasm(arg0);
  try {
    return Output.__construct(wasm.simple_execute(ptr0, len0));

  } finally {
    wasm.__wbindgen_free(ptr0, len0 * 1);
  }
}

function freeOutput(ptr) { wasm.__wbg_output_free(ptr); }
/**
 */
class Output {

  static __construct(ptr) { return new Output(ptr); }

  constructor(ptr) { this.ptr = ptr; }

  free() {
    const ptr = this.ptr;
    this.ptr = 0;
    freeOutput(ptr);
  }
  /**
   * @returns {string}
   */
  stdout() {
    if (this.ptr === 0) {
      throw new Error('Attempt to use a moved value');
    }
    const retptr = globalArgumentPtr();
    wasm.output_stdout(retptr, this.ptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;
  }
  /**
   * @returns {string}
   */
  stderr() {
    if (this.ptr === 0) {
      throw new Error('Attempt to use a moved value');
    }
    const retptr = globalArgumentPtr();
    wasm.output_stderr(retptr, this.ptr);
    const mem = getUint32Memory();
    const rustptr = mem[retptr / 4];
    const rustlen = mem[retptr / 4 + 1];

    const realRet = getStringFromWasm(rustptr, rustlen).slice();
    wasm.__wbindgen_free(rustptr, rustlen * 1);
    return realRet;
  }
}

function __wbindgen_throw(ptr, len) {
  throw new Error(getStringFromWasm(ptr, len));
}

//
// END COPY-PASTE
//

var importObject = {'./basic' : {__wbindgen_throw : __wbindgen_throw}};

// run game
function run() {
  const program = document.getElementById("programTxt").value.trim();
  console.log(program);

  const output = simple_execute(program);

  const stdoutDiv = document.getElementById("output");
  stdoutDiv.textContent =
      "stderr: " + output.stdout() + "\nstderr: " + output.stderr();
}

// load wasm code
WebAssembly.instantiateStreaming(fetch('basic_bg.wasm'), importObject)
    .then(obj => {
      // re-export symbols into the `wasm` object
      var exports = obj.instance.exports;
      Object.getOwnPropertyNames(exports).forEach(
          (val, idx, array) => { wasm[val] = exports[val]; });

      const btn = document.getElementById("runBtn");
      console.log("BASIC interpreter loaded");
      btn.onclick = run;
      btn.disabled = false;
    })
    .catch(e => console.error(e));