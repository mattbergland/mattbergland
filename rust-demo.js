const canvas = document.querySelector('#tracerCanvas');
const ctx = canvas.getContext('2d');
const loading = document.querySelector('#loadingMessage');
const sceneSelect = document.querySelector('#sceneSelect');
const scaleSelect = document.querySelector('#scaleSelect');
const samplesInput = document.querySelector('#samplesInput');
const resetButton = document.querySelector('#resetButton');
const sampleCount = document.querySelector('#sampleCount');
const passTime = document.querySelector('#passTime');
const rayRate = document.querySelector('#rayRate');
let wasm, width = 480, height = 270, maxSamples = 120, yaw = Math.PI / 2, pitch = 0.12;
let dragging = false, lastX = 0, lastY = 0, running = false;

async function loadWasm() {
    const response = await fetch('rust-demo.wasm');
    let result;
    try { result = await WebAssembly.instantiateStreaming(response.clone(), {}); }
    catch (error) { result = await WebAssembly.instantiate(await response.arrayBuffer(), {}); }
    wasm = result.instance;
    configure();
    loading.remove();
    schedule();
}
function configure() {
    const scale = Number(scaleSelect.value);
    width = Math.max(160, Math.round(640 * scale));
    height = Math.round(width * 9 / 16);
    canvas.width = width; canvas.height = height;
    wasm.exports.init(width, height, Number(sceneSelect.value));
    wasm.exports.set_orbit(yaw, pitch);
    sampleCount.textContent = '0';
}
function reset() { wasm.exports.reset(); sampleCount.textContent = '0'; }
function schedule() {
    if (!running) {
        running = true;
        requestAnimationFrame(loop);
    }
}
function loop() {
    if (!wasm) return;
    const before = performance.now();
    const count = Number(wasm.exports.render_pass());
    const ptr = Number(wasm.exports.framebuffer_ptr());
    // Re-acquire this view after every pass: Rust may move linear-memory allocations.
    const pixels = new Uint8ClampedArray(wasm.exports.memory.buffer, ptr, width * height * 4);
    ctx.putImageData(new ImageData(pixels, width, height), 0, 0);
    const elapsed = performance.now() - before;
    sampleCount.textContent = count;
    passTime.textContent = elapsed.toFixed(1);
    rayRate.textContent = (Number(wasm.exports.ray_count()) / elapsed / 1000).toFixed(1);
    if (count < maxSamples) requestAnimationFrame(loop);
    else running = false;
}
function orbit(event) {
    if (!dragging) return;
    yaw += (event.clientX - lastX) * 0.008; pitch = Math.max(-0.8, Math.min(0.8, pitch + (event.clientY - lastY) * 0.006));
    lastX = event.clientX; lastY = event.clientY; wasm.exports.set_orbit(yaw, pitch); sampleCount.textContent = '0'; schedule();
}
canvas.addEventListener('pointerdown', e => { dragging = true; lastX = e.clientX; lastY = e.clientY; try { canvas.setPointerCapture(e.pointerId); } catch (_) {} });
canvas.addEventListener('pointermove', orbit);
canvas.addEventListener('pointerup', () => { dragging = false; });
resetButton.addEventListener('click', () => { maxSamples = Math.max(1, Number(samplesInput.value) || 120); reset(); schedule(); });
sceneSelect.addEventListener('change', () => { configure(); schedule(); });
scaleSelect.addEventListener('change', () => { configure(); schedule(); });
samplesInput.addEventListener('change', () => { maxSamples = Math.max(1, Number(samplesInput.value) || 120); });
loadWasm().catch(error => { loading.textContent = 'The Rust renderer could not load. Please refresh and try again.'; console.error(error); });
