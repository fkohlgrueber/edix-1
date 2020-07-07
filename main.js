import init, { run_app } from './pkg/edix_1.js';
async function main() {
   await init('/pkg/edix_1_bg.wasm');
   run_app();
}
main()