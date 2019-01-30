// We need to load out was asynchronously. Therefore this here acts as a shim and `index.js`
// can do normal imports.
import("./index.js").catch(e => console.error("Error importing wasm: ", e));
