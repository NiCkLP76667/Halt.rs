# Halt.rs API Documentation

## Overview

Halt.rs provides a comprehensive REST API for controlling multi-agent systems. The API allows you to manage circuit breakers, backpressure queues, audit logging, and topology mapping.

## Base URL

```
http://localhost:8080
```

## Authentication

Currently, no authentication is required. In production deployments, consider adding API key authentication.

## Endpoints

### Health Check

#### GET /health

Returns the health status of the Halt.rs server.

**Response:**
```json
{
  "status": "healthy",
  "version": "0.1.0",
  "uptime": 0
}
```

### Circuit Breaker

#### GET /api/v1/circuit-breaker/{from}/{to}

Get the circuit breaker status between two agents.

**Parameters:**
- `from` (string): Source agent identifier
- `to` (string): Destination agent identifier

**Response:**
```json
{
  "from": "AgentA",
  "to": "AgentB",
  "status": "OK",
  "calls_in_window": 0
}
```

#### POST /api/v1/circuit-breaker/{from}/{to}

Register a call between agents with the circuit breaker.

**Parameters:**
- `from` (string): Source agent identifier
- `to` (string): Destination agent identifier

**Request Body:**
```json
{
  "is_terminal": false
}
```

**Response:**
```json
{
  "status": "registered",
  "from": "AgentA",
  "to": "AgentB",
  "is_terminal": false
}
```

### Backpressure Queue

#### POST /api/v1/queue/push

Push a task to the backpressure queue.

**Request Body:**
```json
{
  "name": "High Priority Reasoning",
  "priority": 2,
  "payload": "{\"type\": \"reasoning\", \"model\": \"gpt-4\"}"
}
```

**Response:**
```json
{
  "status": "enqueued"
}
```

**Error Response:**
```json
{
  "error": "Queue at capacity"
}
```

#### POST /api/v1/queue/pop

Pop the highest priority task from the queue.

**Response (success):**
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "High Priority Reasoning",
  "priority": 2,
  "payload": "{\"type\": \"reasoning\", \"model\": \"gpt-4\"}"
}
```

**Response (empty queue):**
```json
null
```

### Audit Logging & Topology

#### GET /api/v1/topology

Get the current swarm topology map as JSON.

**Response:**
```json
{
  "AgentA": [
    {
      "from": "AgentA",
      "to": "AgentB",
      "token_path": ["token_abc", "token_def"],
      "timestamp": "2024-01-01T12:00:00Z"
    }
  ]
}
```

#### POST /api/v1/interactions

Log an interaction between agents.

**Request Body:**
```json
{
  "from": "AgentA",
  "to": "AgentB",
  "token_path": ["token_abc", "token_def"]
}
```

**Response:**
```json
{
  "status": "logged"
}
```

### WebSocket

#### WS /ws

WebSocket endpoint for real-time updates.

**Message Format:**
```json
{
  "type": "topology_update",
  "data": {
    "agents": ["AgentA", "AgentB"],
    "edges": []
  }
}
```

## Error Responses

All endpoints may return error responses in the following format:

```json
{
  "error": "Error message description"
}
```

**HTTP Status Codes:**
- `200`: Success
- `400`: Bad Request (invalid parameters)
- `404`: Not Found
- `429`: Too Many Requests
- `500`: Internal Server Error

## Rate Limiting

- Circuit breaker calls: Limited by the configured threshold and window
- API requests: Currently no rate limiting (consider adding in production)

## Data Types

### Priority Levels
- `0`: Low priority (logging, formatting)
- `1`: Medium priority (standard tasks)
- `2`: High priority (reasoning, Boss Agent)

### Agent Status
- `Processing`: Agent is actively processing
- `Terminal`: Agent has reached a terminal state
- `CoolingDown`: Agent is in cooling down period due to breaker trip

## SDK Examples

### Rust
```rust
use halt::{CircuitBreaker, BackpressureQueue, AuditLogger};

let breaker = CircuitBreaker::new(5, 30);
let queue = BackpressureQueue::new(1000);
let logger = AuditLogger::new();

// Register a call
breaker.register_call("AgentA", "AgentB", false)?;
```

### Python
```python
from halt import HaltClient, Priority

client = HaltClient()

# Push a task
client.push_task("Reasoning Task", Priority.HIGH, '{"model": "gpt-4"}')

# Get topology
topology = client.get_topology()
```

### TypeScript
```typescript
import { HaltRestClient } from 'halt-js';

const client = new HaltRestClient('http://localhost:8080');

// Register call
await client.registerCall('AgentA', 'AgentB', false);
```

### Java
```java
HaltProxy proxy = new HaltProxy();

// Check breaker
String status = proxy.checkBreaker("AgentA", "AgentB");
```

### Go
```go
client := halt.NewClient("http://localhost:8080")

// Push task
resp, err := client.PushTask("Reasoning", halt.PriorityHigh, `{"model": "gpt-4"}`)
```

## Configuration

The API behavior can be configured via environment variables:

- `HALT_CIRCUIT_THRESHOLD`: Circuit breaker threshold (default: 5)
- `HALT_CIRCUIT_WINDOW_SECONDS`: Circuit breaker window (default: 30)
- `HALT_QUEUE_CAPACITY`: Queue capacity (default: 1000)
- `HALT_DATABASE_PATH`: Database file path (default: in-memory)

## Monitoring

Monitor your Halt.rs deployment using:

- `/health` endpoint for health checks
- WebSocket `/ws` for real-time updates
- Logs for debugging
- Database queries for historical data

## Best Practices

1. **Circuit Breaker**: Set appropriate thresholds based on your agent communication patterns
2. **Queue Management**: Monitor queue size to prevent memory issues
3. **Audit Logging**: Regularly review topology maps to identify communication patterns
4. **Error Handling**: Implement proper error handling in your client code
5. **Rate Limiting**: Consider adding rate limiting for production deployments

## Troubleshooting

### Common Issues

1. **Circuit breaker always trips**: Check your threshold and window settings
2. **Queue fills up quickly**: Increase capacity or reduce task submission rate
3. **WebSocket connections fail**: Check firewall settings and CORS configuration
4. **Database errors**: Ensure proper permissions and disk space

### Debug Mode

Enable debug logging by setting:
```bash
export HALT_LOG_LEVEL=debug
```

## Support

For API support, check the GitHub issues or documentation at [halt.rs/docs](https://halt.rs/docs).
