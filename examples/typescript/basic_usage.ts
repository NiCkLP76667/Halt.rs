// Halt.rs TypeScript Example
// Demonstrates using Halt.rs from TypeScript/JavaScript applications

import { init, halt_check_loop } from '../pkg/halt.js';

async function main() {
    console.log('Halt.rs TypeScript Example');

    // Initialize WASM
    await init();
    console.log('WASM initialized');

    // Basic WASM usage
    console.log('\n=== WASM Direct Usage ===');
    try {
        const result = halt_check_loop('AgentA', 'AgentB');
        console.log('WASM result:', result);
    } catch (error) {
        console.error('WASM error:', error);
    }

    // REST API Client Example
    console.log('\n=== REST API Client Example ===');

    const haltClient = new HaltRestClient('http://localhost:8080');

    try {
        // Health check
        const health = await haltClient.healthCheck();
        console.log('Server health:', health);

        // Circuit breaker
        console.log('\n--- Circuit Breaker ---');
        const status = await haltClient.getCircuitBreakerStatus('AgentA', 'AgentB');
        console.log('Breaker status:', status);

        // Register calls
        for (let i = 0; i < 3; i++) {
            const isTerminal = i === 2;
            const result = await haltClient.registerCall('AgentA', 'AgentB', isTerminal);
            console.log(`Call ${i + 1}:`, result);
        }

        // Backpressure queue
        console.log('\n--- Backpressure Queue ---');

        // Push high priority task
        const pushResult1 = await haltClient.pushTask(
            'High Priority Reasoning',
            2, // Priority.HIGH
            JSON.stringify({
                type: 'reasoning',
                model: 'gpt-4',
                prompt: 'Analyze this TypeScript code'
            })
        );
        console.log('Push high priority task:', pushResult1);

        // Push low priority task
        const pushResult2 = await haltClient.pushTask(
            'Low Priority Logging',
            0, // Priority.LOW
            JSON.stringify({
                type: 'logging',
                message: 'User interaction logged'
            })
        );
        console.log('Push low priority task:', pushResult2);

        // Pop tasks
        console.log('Popping tasks:');
        for (let i = 0; i < 2; i++) {
            const task = await haltClient.popTask();
            if (task) {
                console.log(`Task ${i + 1}:`, task.name, `(priority: ${task.priority})`);
            } else {
                console.log('No more tasks');
            }
        }

        // Audit logging
        console.log('\n--- Audit Logging ---');

        const logResult1 = await haltClient.logInteraction(
            'AgentA',
            'AgentB',
            ['token_abc', 'token_def']
        );
        console.log('Log interaction 1:', logResult1);

        const logResult2 = await haltClient.logInteraction(
            'AgentB',
            'AgentC',
            ['token_ghi']
        );
        console.log('Log interaction 2:', logResult2);

        // Get topology
        const topology = await haltClient.getTopology();
        console.log('Current topology:', topology);

    } catch (error) {
        console.error('API error:', error.message);
    }

    // WebSocket Example
    console.log('\n=== WebSocket Example ===');

    try {
        const wsClient = new HaltWebSocketClient('ws://localhost:8080/ws');

        wsClient.onMessage((message) => {
            console.log('WebSocket message:', message);
        });

        // Connect (in real usage, this would be awaited)
        console.log('WebSocket connection example (would connect in real usage)');

    } catch (error) {
        console.error('WebSocket error:', error);
    }

    console.log('\nHalt.rs TypeScript example completed!');
}

// REST API Client Class
class HaltRestClient {
    constructor(private baseUrl: string) {}

    async healthCheck() {
        return this.request('GET', '/health');
    }

    async getCircuitBreakerStatus(from: string, to: string) {
        return this.request('GET', `/api/v1/circuit-breaker/${from}/${to}`);
    }

    async registerCall(from: string, to: string, isTerminal: boolean) {
        return this.request('POST', `/api/v1/circuit-breaker/${from}/${to}`, { is_terminal: isTerminal });
    }

    async pushTask(name: string, priority: number, payload: string) {
        return this.request('POST', '/api/v1/queue/push', { name, priority, payload });
    }

    async popTask() {
        return this.request('POST', '/api/v1/queue/pop');
    }

    async getTopology() {
        return this.request('GET', '/api/v1/topology');
    }

    async logInteraction(from: string, to: string, tokenPath: string[]) {
        return this.request('POST', '/api/v1/interactions', { from, to, token_path: tokenPath });
    }

    private async request(method: string, endpoint: string, data?: any) {
        const url = `${this.baseUrl}${endpoint}`;
        const config: RequestInit = {
            method,
            headers: {
                'Content-Type': 'application/json',
            },
        };

        if (data) {
            config.body = JSON.stringify(data);
        }

        const response = await fetch(url, config);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }

        return response.json();
    }
}

// WebSocket Client Class
class HaltWebSocketClient {
    private ws: WebSocket | null = null;

    constructor(private url: string) {}

    connect(): Promise<void> {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.url);

            this.ws.onopen = () => {
                console.log('WebSocket connected');
                resolve();
            };

            this.ws.onerror = (error) => {
                reject(error);
            };

            this.ws.onmessage = (event) => {
                if (this.onMessageCallback) {
                    this.onMessageCallback(event.data);
                }
            };
        });
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
        }
    }

    private onMessageCallback?: (message: string) => void;

    onMessage(callback: (message: string) => void) {
        this.onMessageCallback = callback;
    }

    send(message: string) {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
            this.ws.send(message);
        }
    }
}

// Run the example
main().catch(console.error);
