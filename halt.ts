import init, { halt_check_loop } from "./pkg/halt.js";

async function run() {
    await init();
    
    // Test the Rust circuit breaker logic via WASM
    const result = halt_check_loop("AgentA", "AgentB");
    console.log("Halt JS Proxy Check:", result);
}

run();
