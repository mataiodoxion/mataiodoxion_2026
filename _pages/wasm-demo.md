---
layout: default
title: WASM Rust Demo
description: Running Rust WASM code in Jekyll
permalink: /wasm-demo
---

<div id="wasm-output"></div>
<button id="run-rust">Run Rust Code</button>

<script type="module">
    import init, { greet, add } from '/mataiodoxion_2026/assets/wasm/wasm_module.js';
    
    async function run() {
        // Initialize the WASM module
        await init();
        
        document.getElementById('run-rust').addEventListener('click', () => {
            const result = add(5, 3);
            document.getElementById('wasm-output').innerText = `Rust says: 5 + 3 = ${result}`;
            greet('WASM');
        });
    }
    
    run();
</script>