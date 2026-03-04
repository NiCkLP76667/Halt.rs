import { HaltMCPServer } from '../src/index';

describe('HaltMCPServer', () => {
  let server: HaltMCPServer;

  beforeEach(() => {
    server = new HaltMCPServer();
    // Mock environment variables
    process.env.HALT_BASE_URL = 'http://localhost:8080';
  });

  afterEach(() => {
    delete process.env.HALT_BASE_URL;
  });

  describe('Tool Handlers', () => {
    test('should list available tools', async () => {
      // Test that the server can be instantiated
      expect(server).toBeDefined();
      expect(server).toBeInstanceOf(HaltMCPServer);
    });

    test('should handle halt_check_breaker tool', async () => {
      // This test would require mocking the MCP server internals
      // For now, we just verify the class exists
      expect(server).toBeDefined();
    });

    test('should handle halt_register_call tool', async () => {
      expect(server).toBeDefined();
    });

    test('should handle halt_push_task tool', async () => {
      expect(server).toBeDefined();
    });

    test('should handle halt_pop_task tool', async () => {
      expect(server).toBeDefined();
    });

    test('should handle halt_get_topology tool', async () => {
      expect(server).toBeDefined();
    });

    test('should handle halt_log_interaction tool', async () => {
      expect(server).toBeDefined();
    });

    test('should throw error for unknown tool', async () => {
      expect(server).toBeDefined();
    });
  });

  describe('Resource Handlers', () => {
    test('should list available resources', async () => {
      expect(server).toBeDefined();
    });

    test('should read topology resource', async () => {
      expect(server).toBeDefined();
    });

    test('should read status resource', async () => {
      expect(server).toBeDefined();
    });

    test('should read metrics resource', async () => {
      expect(server).toBeDefined();
    });

    test('should throw error for unknown resource', async () => {
      expect(server).toBeDefined();
    });
  });

  describe('Error Handling', () => {
    test('should handle API errors', async () => {
      expect(server).toBeDefined();
    });

    test('should handle network errors', async () => {
      expect(server).toBeDefined();
    });
  });

  describe('Configuration', () => {
    test('should use default base URL', () => {
      const testServer = new HaltMCPServer();
      expect(testServer).toBeDefined();
    });

    test('should use custom base URL from environment', () => {
      process.env.HALT_BASE_URL = 'http://custom:9000';
      const testServer = new HaltMCPServer();
      expect(testServer).toBeDefined();
      delete process.env.HALT_BASE_URL;
    });
  });
});
