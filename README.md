# 🛑 Halt.rs - Multi-Agent Proxy for Swarm Control

<div align="center">

![Halt.rs](https://img.shields.io/badge/version-0.1.0-blue)
![License](https://img.shields.io/badge/license-MIT-green)
![Rust](https://img.shields.io/badge/rust-1.70+-orange)
![Stars](https://img.shields.io/badge/stars-⭐-ff69b4)

**Prevent runaway AI agent costs before they happen.**

[Features](#-features) • [Quick Start](#-quick-start) • [Documentation](#-documentation) • [Contributing](#-contributing)

</div>

---

## 🎯 The Problem

Building multi-agent AI systems? You've probably faced these nightmares:

- **$500 API Bills**: Agent A calls Agent B, which calls Agent A again... infinite loops that drain your wallet
- **Priority Chaos**: Your "Boss Agent" gets stuck behind hundreds of low-priority logging tasks
- **Debugging Mess**: Which agent started the loop? Who's spamming the system? Good luck finding out

## 🚀 The Solution

**Halt.rs** is a production-grade Rust-based proxy that prevents these disasters before they happen:

### 🛡️ Circuit Breaker
- Monitors inter-agent call frequency
- Trips automatically when Agent A triggers Agent B >5 times in 30 seconds without a terminal state
- Returns "Cooling Down" error to save you from API bills
- Configurable thresholds and time windows

### 📊 Backpressure Queue
- Priority-based task queuing (High/Medium/Low)
- Your "Boss Agent" always gets priority over worker agents
- Automatic throttling during traffic spikes
- Configurable capacity limits

### 🗺️ Audit-Log Transparency
- Real-time swarm topology mapping
- Tracks every "Token Path" so you can see exactly which agent started the loop
- Visual debugging of multi-agent systems
- JSON export for integration with monitoring tools

### 🌐 Multi-Language Support
- **Rust Core**: High-performance, memory-safe implementation
- **Python Bindings**: Full async client with Pydantic models
- **Java Bindings**: JNI-based proxy with comprehensive API
- **TypeScript/JavaScript**: WASM integration + REST client
- **Go Bindings**: High-performance fasthttp client

### 🔌 Model Context Protocol (MCP)
- Native MCP server for IDE integration
- Instant installation in Cursor, VSCode, Zed, JetBrains
- Real-time topology updates via WebSocket
- Comprehensive tool and resource definitions

### 🐳 Enterprise Features
- **Docker**: Multi-stage containerization
- **CI/CD**: GitHub Actions with cross-platform builds
- **REST API**: Full HTTP server for remote control
- **WebSocket**: Real-time monitoring and updates
- **SQLite**: Persistent storage for audit logs and tasks
- **CLI Tool**: Command-line interface for management
- **Plugin System**: Extensible architecture for custom strategies

---

## ✨ Features

### Core Capabilities

| Feature | Description |
|----------|-------------|
| **Circuit Breaker** | Prevents infinite loops between agents with configurable thresholds |
| **Backpressure Queue** | Priority-based task queuing with automatic throttling |
| **Audit Logging** | Real-time swarm topology mapping with token path tracking |
| **REST API** | Full HTTP server for remote control and monitoring |
| **WebSocket** | Real-time updates for topology changes |
| **CLI Tool** | Command-line interface for management operations |
| **MCP Server** | Model Context Protocol for IDE integration |
| **Plugin System** | Extensible architecture for custom implementations |

### Language Bindings

| Language | Package | Status |
|----------|---------|--------|
| **Rust** | `halt` | ✅ Core |
| **Python** | `halt-py` | ✅ Stable |
| **Java** | `halt-java` | ✅ Stable |
| **TypeScript** | `halt-js` | ✅ Stable |
| **Go** | `halt-go` | ✅ Stable |

### Infrastructure

| Component | Description |
|-----------|-------------|
| **Docker** | Multi-stage containerization |
| **CI/CD** | GitHub Actions with cross-platform builds |
| **Build Scripts** | Comprehensive bash scripts for all platforms |
| **Tests** | Jest (TS), pytest (Python), cargo test (Rust) |
| **Benchmarks** | Criterion.rs performance benchmarks |
| **Documentation** | Comprehensive docs and examples |

---

## 🚀 Quick Start

### One-Line Install & Run

**Try Halt.rs instantly:**

```bash
git clone https://github.com/ez0000001000000/Halt.rs && cd Halt.rs && cargo run -- serve --port 8080
```

*Requires Rust installed. Server starts on http://localhost:8080*

### Installation

```bash
# Rust
cargo install halt

# Python
pip install halt-py

# Java
# Add to pom.xml or download JAR

# TypeScript/JavaScript
npm install halt-js
```

### Run Server

```bash
halt serve --port 8080
```

### Basic Usage

#### Rust
```rust
use halt::{CircuitBreaker, BackpressureQueue, AuditLogger};

let breaker = CircuitBreaker::new(5, 30); // 5 calls per 30 seconds
let queue = BackpressureQueue::new(1000);
let logger = AuditLogger::new();

// Register a call
if let Err(err) = breaker.register_call("AgentA", "AgentB", false) {
    println!("Circuit tripped: {}", err);
}
```

#### Python
```python
from halt import HaltClient, Priority

client = HaltClient()
client.push_task("Reasoning", Priority.HIGH, '{"model": "gpt-4"}')
topology = client.get_topology()
```

#### TypeScript
```typescript
import { HaltRestClient } from 'halt-js';

const client = new HaltRestClient('http://localhost:8080');
await client.registerCall('AgentA', 'AgentB', false);
```

#### Java
```java
HaltProxy proxy = new HaltProxy();
String status = proxy.checkBreaker("AgentA", "AgentB");
```

#### Go
```go
client := halt.NewClient("http://localhost:8080")
resp, err := client.PushTask("Reasoning", halt.PriorityHigh, `{"model": "gpt-4"}`)
```

---

## 📚 Documentation

### API Reference

- **REST API**: [docs/api/api.md](docs/api/api.md)
- **CLI Commands**: [docs/cli/commands.md](docs/cli/commands.md)
- **Configuration**: [docs/config/options.md](docs/config/options.md)

### Examples

- **Rust**: [examples/rust/basic_usage.rs](examples/rust/basic_usage.rs)
- **Python**: [examples/python/basic_usage.py](examples/python/basic_usage.py)
- **TypeScript**: [examples/typescript/basic_usage.ts](examples/typescript/basic_usage.ts)
- **Java**: [examples/java/HaltExample.java](examples/java/HaltExample.java)
- **Go**: [examples/go/basic_usage.go](examples/go/basic_usage.go)

### Architecture

- **Core**: [src/core/](src/core/) - Circuit breaker, queue, audit logger
- **API**: [src/api/](src/api/) - REST server and WebSocket
- **CLI**: [src/cli/](src/cli/) - Command-line interface
- **MCP**: [src/mcp/](src/mcp/) - Model Context Protocol server
- **Plugins**: [src/plugins.rs](src/plugins.rs) - Extensible plugin system

---

## 🔧 Configuration

Halt.rs can be configured via environment variables:

```bash
export HALT_CIRCUIT_THRESHOLD=5
export HALT_CIRCUIT_WINDOW_SECONDS=30
export HALT_QUEUE_CAPACITY=1000
export HALT_DATABASE_PATH=./halt.db
```

See [config/default.env](config/default.env) for all options.

---

## 🧪 Testing

```bash
# Run all tests
./scripts/build/build.sh test

# Run specific test suites
cargo test                    # Rust tests
cd mcp-server && npm test     # MCP server tests
cd bindings/python && pytest  # Python tests
```

---

## 🐳 Docker

```bash
# Build
docker build -t halt-rs:latest -f docker/Dockerfile .

# Run
docker run -d --name halt -p 8080:8080 halt-rs:latest
```

---

## 📖 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone
git clone https://github.com/halt-rs/halt.git
cd halt

# Build
./scripts/build/build.sh all

# Test
./scripts/build/build.sh test

# Run dev server
cargo run -- serve
```

---

## 🎓 Use Cases

### 1. Multi-Agent Orchestration
Prevent runaway loops in complex agent systems

### 2. API Cost Management
Stop infinite recursive calls before they drain your budget

### 3. Priority-Based Task Processing
Ensure critical reasoning tasks aren't blocked by logging

### 4. System Debugging
Visual topology maps for understanding agent interactions

### 5. Production Monitoring
Real-time metrics and audit logs for observability

---

## 🏗️ Architecture

```
halt-rs/
├── src/                    # Core Rust implementation
│   ├── core/              # Circuit breaker, queue, audit logger
│   ├── api/               # REST API and WebSocket
│   ├── cli/               # Command-line interface
│   └── mcp/               # Model Context Protocol
├── bindings/              # Language bindings
│   ├── python/            # Python bindings
│   ├── java/              # Java bindings
│   ├── typescript/        # TypeScript bindings
│   └── go/                # Go bindings
├── mcp-server/            # MCP TypeScript server
├── docker/                 # Docker containerization
├── docs/                   # Documentation
├── examples/              # Code examples
├── scripts/                # Build and deployment
└── tests/                  # Integration tests
```

---

## 📊 Performance

- **Circuit Breaker**: <1μs per call
- **Queue Operations**: <100μs push/pop
- **Audit Logging**: <500μs per interaction
- **Memory Usage**: <50MB idle, <200MB with 10k tasks
- **Throughput**: 10k+ operations/second

See [benches/performance.rs](benches/performance.rs) for benchmarks.

---

## 🔒 Security

- Input validation on all API endpoints
- SQL injection protection
- Rate limiting support
- No authentication by default (configure for production)

---

## 📝 License

MIT License - see [LICENSE](LICENSE) for details

---

## 🙏 Acknowledgments

Built with:
- **Rust**: Core implementation
- **Actix-web**: HTTP server
- **Model Context Protocol**: IDE integration
- **Tokio**: Async runtime
- **SQLite**: Persistent storage

---

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=halt-rs/halt&type=Date)](https://star-history.com/#halt-rs/halt&Date)

---

## 📞 Support

- **GitHub Issues**: [github.com/halt-rs/halt/issues](https://github.com/halt-rs/halt/issues)
- **Discussions**: [github.com/halt-rs/halt/discussions](https://github.com/halt-rs/halt/discussions)
- **Documentation**: [halt.rs/docs](https://halt.rs/docs)

---

<div align="center">

**[⬆ Back to Top](#-halt-rs---multi-agent-proxy-for-swarm-control)**

Made with ❤️ by the Halt.rs Team

</div>
