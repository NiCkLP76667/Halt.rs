#!/usr/bin/env python3
"""
Halt.rs Python Example
Demonstrates using Halt.rs from Python applications
"""

import asyncio
import json
from halt import HaltClient, Priority, HaltException

async def main():
    print("Halt.rs Python Example")

    # Initialize client
    client = HaltClient(base_url="http://localhost:8080")

    try:
        # Health check
        print("\n=== Health Check ===")
        health = client.health_check()
        print(f"Server health: {health}")

        # Circuit Breaker Example
        print("\n=== Circuit Breaker Example ===")

        # Check breaker status
        status = client.get_circuit_breaker_status("AgentA", "AgentB")
        print(f"Breaker status: {status}")

        # Register calls
        for i in range(3):
            is_terminal = (i == 2)  # Last call is terminal
            result = client.register_call("AgentA", "AgentB", is_terminal)
            print(f"Call {i+1}: {result}")

        # Backpressure Queue Example
        print("\n=== Backpressure Queue Example ===")

        # Push high priority task
        result = client.push_task(
            name="High Priority Reasoning",
            priority=Priority.HIGH,
            payload=json.dumps({
                "type": "reasoning",
                "model": "gpt-4",
                "prompt": "Analyze this multi-agent system"
            })
        )
        print(f"Push high priority task: {result}")

        # Push low priority task
        result = client.push_task(
            name="Low Priority Logging",
            priority=Priority.LOW,
            payload=json.dumps({
                "type": "logging",
                "message": "Background task completed"
            })
        )
        print(f"Push low priority task: {result}")

        # Pop tasks (should get high priority first)
        print("Popping tasks:")
        for i in range(2):
            task = client.pop_task()
            if task:
                print(f"Task {i+1}: {task.name} (priority: {task.priority})")
            else:
                print(f"No more tasks")

        # Audit Logging Example
        print("\n=== Audit Logging Example ===")

        # Log interactions
        result = client.log_interaction(
            from_agent="AgentA",
            to_agent="AgentB",
            token_path=["token_abc", "token_def"]
        )
        print(f"Log interaction: {result}")

        result = client.log_interaction(
            from_agent="AgentB",
            to_agent="AgentC",
            token_path=["token_ghi"]
        )
        print(f"Log interaction: {result}")

        # Get topology
        topology = client.get_topology()
        print(f"Current topology: {json.dumps(topology, indent=2)}")

        # WebSocket Example
        print("\n=== WebSocket Example ===")

        async def on_message(message):
            print(f"WebSocket message: {message}")

        print("Connecting to WebSocket (this would run indefinitely)...")
        print("In a real application, you'd run: await client.websocket_connect(on_message)")

    except HaltException as e:
        print(f"Halt error: {e}")
    except Exception as e:
        print(f"Unexpected error: {e}")

if __name__ == "__main__":
    asyncio.run(main())
