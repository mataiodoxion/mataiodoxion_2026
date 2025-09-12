---
layout: default
title: Binary Protocol API Demo
description: Testing binary protocol addition API
permalink: /api/add
---

<div class="api-demo">
    <h2>Binary Protocol Addition API</h2>
    <p>Protocol: <code>0x01 0x?? 0x??</code> where 0x01 is the addition opcode</p>
    
    <div class="input-section">
        <label for="protocol-input">Enter Protocol String:</label>
        <input type="text" id="protocol-input" value="0x01 0x05 0x03" placeholder="0x01 0x05 0x03">
        <button id="process-btn">Process Request</button>
    </div>
    
    <div class="results">
        <h3>Results:</h3>
        <div id="result-output"></div>
        <div id="error-output" style="color: red;"></div>
    </div>
    
    <div class="examples">
        <h3>Try These Examples:</h3>
        <button class="example-btn" data-example="0x01 0x0A 0x14">Add 10 + 20</button>
        <button class="example-btn" data-example="0x01 0xFF 0x01">Add 255 + 1 (overflow)</button>
        <button class="example-btn" data-example="0x01 0x7F 0x7F">Add 127 + 127</button>
    </div>
</div>

<script type="module">
    import '/mataiodoxion_2026/assets/js/binary-api.js';
    
    const api = new window.BinaryAPI();
    const input = document.getElementById('protocol-input');
    const processBtn = document.getElementById('process-btn');
    const resultOutput = document.getElementById('result-output');
    const errorOutput = document.getElementById('error-output');
    
    async function processRequest() {
        try {
            errorOutput.textContent = '';
            const inputValue = input.value.trim();
            
            if (!inputValue) {
                throw new Error('Please enter a protocol string');
            }
            
            const result = await api.processRequest(inputValue);
            const hexResult = api.resultToHex(result);
            
            // Parse the result for display
            if (result.length === 2 && result[0] === 0x01) {
                const sum = result[1];
                resultOutput.innerHTML = `
                    <strong>Success!</strong><br>
                    Raw Result: <code>${hexResult}</code><br>
                    Decoded: Addition result = ${sum}
                `;
            } else if (result.length === 1 && result[0] === 0xFF) {
                resultOutput.innerHTML = `
                    <strong>Error from API:</strong><br>
                    Raw Result: <code>${hexResult}</code><br>
                    Message: Invalid request or unknown opcode
                `;
            } else {
                resultOutput.innerHTML = `
                    <strong>Unexpected Result:</strong><br>
                    Raw Result: <code>${hexResult}</code>
                `;
            }
        } catch (error) {
            errorOutput.textContent = `Error: ${error.message}`;
            resultOutput.textContent = '';
        }
    }
    
    processBtn.addEventListener('click', processRequest);
    
    // Handle example buttons
    document.querySelectorAll('.example-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            input.value = btn.dataset.example;
            processRequest();
        });
    });
    
    // Process on Enter key
    input.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            processRequest();
        }
    });
</script>

<style>
.api-demo {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
}

.input-section {
    margin: 20px 0;
    padding: 20px;
    background: #f5f5f5;
    border-radius: 5px;
}

.input-section label {
    display: block;
    margin-bottom: 10px;
    font-weight: bold;
}

.input-section input {
    width: 300px;
    padding: 8px;
    margin-right: 10px;
    font-family: monospace;
}

.input-section button {
    padding: 8px 15px;
    background: #007cba;
    color: white;
    border: none;
    border-radius: 3px;
    cursor: pointer;
}

.results {
    margin: 20px 0;
    padding: 20px;
    background: #f9f9f9;
    border-radius: 5px;
    min-height: 80px;
}

.examples {
    margin: 20px 0;
}

.example-btn {
    margin: 5px;
    padding: 5px 10px;
    background: #28a745;
    color: white;
    border: none;
    border-radius: 3px;
    cursor: pointer;
    font-family: monospace;
}

.example-btn:hover {
    background: #218838;
}
</style>