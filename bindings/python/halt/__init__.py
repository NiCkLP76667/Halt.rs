"""
Halt.py - Python bindings for Halt.rs multi-agent proxy

This module provides Python bindings for the Halt.rs system, allowing
Python applications to integrate with the circuit breaker, backpressure
queue, and audit logging functionality.
"""

import asyncio
import json
import logging
from typing import Dict, List, Optional, Any, Union
import requests
import websockets
from pydantic import BaseModel, Field

__version__ = "0.1.0"
__all__ = [
    "HaltClient",
    "CircuitBreaker",
    "BackpressureQueue",
    "AuditLogger",
    "HaltException",
    "Priority",
]

logger = logging.getLogger(__name__)


class HaltException(Exception):
    """Base exception for Halt.py operations."""
    pass


class Priority:
    """Priority levels for tasks in the backpressure queue."""
    LOW = 0
    MEDIUM = 1
    HIGH = 2


class Task(BaseModel):
    """Represents a task in the backpressure queue."""
    id: str = Field(..., description="Unique task identifier")
    name: str = Field(..., description="Task name/description")
    priority: int = Field(..., ge=0, le=2, description="Task priority (0-2)")
    payload: str = Field(..., description="Task payload data")


class Interaction(BaseModel):
    """Represents an agent interaction for logging."""
    from_agent: str = Field(..., description="Source agent identifier")
    to_agent: str = Field(..., description="Destination agent identifier")
    token_path: List[str] = Field(default_factory=list, description="Token path identifiers")


class HaltClient:
    """
    Main client for interacting with Halt.rs server.

    Provides methods to control circuit breakers, manage queues,
    and access audit logs.
    """

    def __init__(self, base_url: str = "http://localhost:8080", timeout: float = 5.0):
        """
        Initialize the Halt client.

        Args:
            base_url: Base URL of the Halt.rs server
            timeout: Request timeout in seconds
        """
        self.base_url = base_url.rstrip("/")
        self.timeout = timeout
        self._session = requests.Session()

    def _make_request(self, method: str, endpoint: str, **kwargs) -> Dict[str, Any]:
        """Make an HTTP request to the Halt server."""
        url = f"{self.base_url}{endpoint}"
        kwargs.setdefault("timeout", self.timeout)

        try:
            response = self._session.request(method, url, **kwargs)
            response.raise_for_status()
            return response.json()
        except requests.RequestException as e:
            raise HaltException(f"Request failed: {e}") from e

    def health_check(self) -> Dict[str, Any]:
        """Check server health."""
        return self._make_request("GET", "/health")

    def get_circuit_breaker_status(self, from_agent: str, to_agent: str) -> Dict[str, Any]:
        """Get circuit breaker status between two agents."""
        return self._make_request("GET", f"/api/v1/circuit-breaker/{from_agent}/{to_agent}")

    def register_call(self, from_agent: str, to_agent: str, is_terminal: bool = False) -> Dict[str, Any]:
        """Register a call between agents."""
        data = {"is_terminal": is_terminal}
        return self._make_request("POST", f"/api/v1/circuit-breaker/{from_agent}/{to_agent}", json=data)

    def push_task(self, name: str, priority: int, payload: str) -> Dict[str, Any]:
        """Push a task to the backpressure queue."""
        data = {
            "name": name,
            "priority": priority,
            "payload": payload,
        }
        return self._make_request("POST", "/api/v1/queue/push", json=data)

    def pop_task(self) -> Optional[Task]:
        """Pop the highest priority task from the queue."""
        result = self._make_request("POST", "/api/v1/queue/pop")
        if result is None:
            return None
        return Task(**result)

    def get_topology(self) -> Dict[str, Any]:
        """Get the current swarm topology map."""
        return self._make_request("GET", "/api/v1/topology")

    def log_interaction(self, from_agent: str, to_agent: str, token_path: List[str]) -> Dict[str, Any]:
        """Log an interaction between agents."""
        data = {
            "from": from_agent,
            "to": to_agent,
            "token_path": token_path,
        }
        return self._make_request("POST", "/api/v1/interactions", json=data)

    async def websocket_connect(self, on_message=None):
        """
        Connect to the WebSocket for real-time updates.

        Args:
            on_message: Callback function for incoming messages
        """
        uri = f"ws://localhost:8080/ws"  # TODO: Make configurable

        async with websockets.connect(uri) as websocket:
            logger.info("Connected to Halt WebSocket")

            async for message in websocket:
                if on_message:
                    try:
                        data = json.loads(message)
                        await on_message(data)
                    except json.JSONDecodeError:
                        logger.warning(f"Invalid JSON message: {message}")
                else:
                    logger.info(f"Received: {message}")


class CircuitBreaker:
    """Circuit breaker for managing agent call patterns."""

    def __init__(self, client: HaltClient, threshold: int = 5, window_seconds: int = 30):
        self.client = client
        self.threshold = threshold
        self.window_seconds = window_seconds

    def check_call(self, from_agent: str, to_agent: str) -> Dict[str, Any]:
        """Check if a call should be allowed."""
        return self.client.get_circuit_breaker_status(from_agent, to_agent)

    def register_call(self, from_agent: str, to_agent: str, is_terminal: bool = False) -> Dict[str, Any]:
        """Register a call and potentially trip the breaker."""
        return self.client.register_call(from_agent, to_agent, is_terminal)


class BackpressureQueue:
    """Backpressure queue for managing task priorities."""

    def __init__(self, client: HaltClient, capacity: int = 1000):
        self.client = client
        self.capacity = capacity

    def push(self, name: str, priority: int, payload: str) -> Dict[str, Any]:
        """Push a task to the queue."""
        return self.client.push_task(name, priority, payload)

    def pop(self) -> Optional[Task]:
        """Pop the highest priority task."""
        return self.client.pop_task()


class AuditLogger:
    """Audit logger for tracking agent interactions."""

    def __init__(self, client: HaltClient):
        self.client = client

    def get_topology(self) -> Dict[str, Any]:
        """Get the current topology map."""
        return self.client.get_topology()

    def log_interaction(self, from_agent: str, to_agent: str, token_path: List[str]) -> Dict[str, Any]:
        """Log an agent interaction."""
        return self.client.log_interaction(from_agent, to_agent, token_path)
