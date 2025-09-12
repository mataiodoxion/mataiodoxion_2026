import init, { handle_binary_protocol, parse_protocol_string } from '/mataiodoxion_2026/assets/wasm/wasm_module.js';

class BinaryAPI {
    constructor() {
        this.initialized = false;
    }

    async init() {
        if (!this.initialized) {
            await init();
            this.initialized = true;
        }
    }

    // Handle the binary protocol
    async processRequest(data) {
        await this.init();

        let bytes;
        if (typeof data === 'string') {
            // Parse hex string format: "0x01 0x05 0x03"
            bytes = parse_protocol_string(data);
        } else if (data instanceof Uint8Array) {
            bytes = Array.from(data);
        } else {
            throw new Error('Invalid data format');
        }

        const result = handle_binary_protocol(bytes);
        return new Uint8Array(result);
    }

    // Convert result to hex string for display
    resultToHex(result) {
        return Array.from(result)
            .map(byte => `0x${byte.toString(16).padStart(2, '0').toUpperCase()}`)
            .join(' ');
    }
}

// Export for use in other scripts
window.BinaryAPI = BinaryAPI;