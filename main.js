import * as wasm from './pkg';

let result = wasm.add(Math.floor(Math.random() * 11), Math.floor(Math.random() * 11));

console.log(result);
document.write(result)
