#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ErrorCode,
  ListResourcesRequestSchema,
  ListToolsRequestSchema,
  McpError,
  ReadResourceRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";

class HaltMCPServer {
  private server: Server;
  private haltBaseUrl: string;

  constructor() {
    this.haltBaseUrl = process.env.HALT_BASE_URL || "http://localhost:8080";
    this.server = new Server(
      {
        name: "halt-mcp-server",
        version: "0.1.0",
      },
      {
        capabilities: {
          tools: {},
          resources: {},
        },
      }
    );

    this.setupToolHandlers();
    this.setupResourceHandlers();
  }

  private setupToolHandlers() {
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      return {
        tools: [
          {
            name: "halt_check_breaker",
            description: "Check circuit breaker status between two agents",
            inputSchema: {
              type: "object",
              properties: {
                from: { type: "string", description: "Source agent identifier" },
                to: { type: "string", description: "Destination agent identifier" },
              },
              required: ["from", "to"],
            },
          },
          {
            name: "halt_register_call",
            description: "Register a call between agents with the circuit breaker",
            inputSchema: {
              type: "object",
              properties: {
                from: { type: "string", description: "Source agent identifier" },
                to: { type: "string", description: "Destination agent identifier" },
                isTerminal: { type: "boolean", description: "Whether the call reached a terminal state" },
              },
              required: ["from", "to", "isTerminal"],
            },
          },
          {
            name: "halt_push_task",
            description: "Push a task to the backpressure queue",
            inputSchema: {
              type: "object",
              properties: {
                name: { type: "string", description: "Task name" },
                priority: { type: "number", enum: [0, 1, 2], description: "Priority level (0=Low, 1=Medium, 2=High)" },
                payload: { type: "string", description: "Task payload data" },
              },
              required: ["name", "priority", "payload"],
            },
          },
          {
            name: "halt_pop_task",
            description: "Pop the highest priority task from the queue",
            inputSchema: {
              type: "object",
              properties: {},
            },
          },
          {
            name: "halt_get_topology",
            description: "Get the current swarm topology map",
            inputSchema: {
              type: "object",
              properties: {},
            },
          },
          {
            name: "halt_log_interaction",
            description: "Log an interaction between agents",
            inputSchema: {
              type: "object",
              properties: {
                from: { type: "string", description: "Source agent identifier" },
                to: { type: "string", description: "Destination agent identifier" },
                tokenPath: { type: "array", items: { type: "string" }, description: "Token path identifiers" },
              },
              required: ["from", "to", "tokenPath"],
            },
          },
        ],
      };
    });

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      const { name, arguments: args } = request.params;

      if (!args) {
        throw new McpError(ErrorCode.InvalidParams, "Arguments are required");
      }

      try {
        switch (name) {
          case "halt_check_breaker":
            const breakerResponse = await this.callHaltAPI(`/api/v1/circuit-breaker/${args.from}/${args.to}`);
            return {
              content: [{ type: "text", text: JSON.stringify(breakerResponse.data, null, 2) }],
            };

          case "halt_register_call":
            const registerResponse = await this.callHaltAPI(`/api/v1/circuit-breaker/${args.from}/${args.to}`, "POST", {
              is_terminal: args.isTerminal,
            });
            return {
              content: [{ type: "text", text: JSON.stringify(registerResponse.data, null, 2) }],
            };

          case "halt_push_task":
            const pushResponse = await this.callHaltAPI("/api/v1/queue/push", "POST", {
              name: args.name,
              priority: args.priority,
              payload: args.payload,
            });
            return {
              content: [{ type: "text", text: JSON.stringify(pushResponse.data, null, 2) }],
            };

          case "halt_pop_task":
            const popResponse = await this.callHaltAPI("/api/v1/queue/pop", "POST");
            return {
              content: [{ type: "text", text: JSON.stringify(popResponse.data, null, 2) }],
            };

          case "halt_get_topology":
            const topologyResponse = await this.callHaltAPI("/api/v1/topology");
            return {
              content: [{ type: "text", text: JSON.stringify(topologyResponse.data, null, 2) }],
            };

          case "halt_log_interaction":
            const logResponse = await this.callHaltAPI("/api/v1/interactions", "POST", {
              from: args.from,
              to: args.to,
              token_path: args.tokenPath,
            });
            return {
              content: [{ type: "text", text: JSON.stringify(logResponse.data, null, 2) }],
            };

          default:
            throw new McpError(ErrorCode.MethodNotFound, `Unknown tool: ${name}`);
        }
      } catch (error) {
        const err = error as any;
        if (err.response) {
          return {
            content: [{ type: "text", text: `Halt API Error: ${err.response.status} - ${err.response.data}` }],
            isError: true,
          };
        }
        throw new McpError(ErrorCode.InternalError, `Tool execution failed: ${err.message}`);
      }
    });
  }

  private setupResourceHandlers() {
    this.server.setRequestHandler(ListResourcesRequestSchema, async () => {
      return {
        resources: [
          {
            uri: "halt://topology",
            name: "Swarm Topology Map",
            description: "Real-time map of agent interactions and relationships",
            mimeType: "application/json",
          },
          {
            uri: "halt://status",
            name: "Halt System Status",
            description: "Current status of all Halt.rs components",
            mimeType: "application/json",
          },
          {
            uri: "halt://metrics",
            name: "System Metrics",
            description: "Performance metrics and statistics",
            mimeType: "application/json",
          },
        ],
      };
    });

    this.server.setRequestHandler(ReadResourceRequestSchema, async (request) => {
      const { uri } = request.params;

      try {
        let data;
        switch (uri) {
          case "halt://topology":
            const topologyResponse = await this.callHaltAPI("/api/v1/topology");
            data = topologyResponse.data;
            break;
          case "halt://status":
            const statusResponse = await this.callHaltAPI("/health");
            data = statusResponse.data;
            break;
          case "halt://metrics":
            // Mock metrics for now
            data = {
              circuit_breaker_trips: 0,
              queue_size: 0,
              total_interactions: 0,
              uptime_seconds: 0,
            };
            break;
          default:
            throw new McpError(ErrorCode.InvalidRequest, `Unknown resource: ${uri}`);
        }

        return {
          contents: [
            {
              uri,
              mimeType: "application/json",
              text: JSON.stringify(data, null, 2),
            },
          ],
        };
      } catch (error) {
        throw new McpError(ErrorCode.InternalError, `Resource read failed: ${error.message}`);
      }
    });
  }

  private async callHaltAPI(endpoint: string, method: string = "GET", data?: any) {
    const axios = await import("axios");
    const url = `${this.haltBaseUrl}${endpoint}`;

    return axios.default({
      method,
      url,
      data,
      timeout: 5000,
    });
  }

  async run() {
    const transport = new StdioServerTransport();
    await this.server.connect(transport);
    console.error("Halt MCP server running on stdio");
  }
}

export { HaltMCPServer };

const server = new HaltMCPServer();
server.run().catch((error: Error) => {
  console.error("MCP server error:", error);
  process.exit(1);
});
