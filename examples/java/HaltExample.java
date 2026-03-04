// Halt.rs Java Example
// Demonstrates using Halt.rs from Java applications

public class HaltExample {
    public static void main(String[] args) {
        System.out.println("Halt.rs Java Example");

        // Initialize Halt proxy
        HaltProxy proxy = new HaltProxy();

        try {
            // Health check
            System.out.println("\n=== Health Check ===");
            String health = proxy.getVersion();
            System.out.println("Halt version: " + health);

            String status = proxy.getStatus();
            System.out.println("System status: " + status);

            // Circuit Breaker Example
            System.out.println("\n=== Circuit Breaker Example ===");

            // Check breaker status
            String breakerStatus = proxy.checkBreaker("AgentA", "AgentB");
            System.out.println("Breaker status: " + breakerStatus);

            // Register calls
            for (int i = 0; i < 3; i++) {
                boolean isTerminal = (i == 2); // Last call is terminal
                String result = proxy.registerCall("AgentA", "AgentB", isTerminal);
                System.out.println("Call " + (i + 1) + ": " + result);
            }

            // Reset breaker
            String resetResult = proxy.resetBreaker("AgentA", "AgentB");
            System.out.println("Reset breaker: " + resetResult);

            // Backpressure Queue Example
            System.out.println("\n=== Backpressure Queue Example ===");

            // Push high priority task
            String taskId1 = java.util.UUID.randomUUID().toString();
            String pushResult1 = proxy.pushTask(
                taskId1,
                "High Priority Reasoning",
                2, // Priority.HIGH
                "{\"type\": \"reasoning\", \"model\": \"gpt-4\", \"prompt\": \"Analyze this Java code\"}"
            );
            System.out.println("Push high priority task: " + pushResult1);

            // Push low priority task
            String taskId2 = java.util.UUID.randomUUID().toString();
            String pushResult2 = proxy.pushTask(
                taskId2,
                "Low Priority Logging",
                0, // Priority.LOW
                "{\"type\": \"logging\", \"message\": \"Background task completed\"}"
            );
            System.out.println("Push low priority task: " + pushResult2);

            // Check queue size
            int queueSize = proxy.getQueueSize();
            System.out.println("Queue size: " + queueSize);

            // Pop tasks
            System.out.println("Popping tasks:");
            for (int i = 0; i < Math.min(queueSize, 2); i++) {
                String taskJson = proxy.popTask();
                if (!"null".equals(taskJson)) {
                    System.out.println("Task " + (i + 1) + ": " + taskJson);
                } else {
                    System.out.println("No more tasks");
                }
            }

            // Audit Logging Example
            System.out.println("\n=== Audit Logging Example ===");

            // Log interactions
            String logResult1 = proxy.logInteraction(
                "AgentA",
                "AgentB",
                "[\"token_abc\", \"token_def\"]"
            );
            System.out.println("Log interaction 1: " + logResult1);

            String logResult2 = proxy.logInteraction(
                "AgentB",
                "AgentC",
                "[\"token_ghi\"]"
            );
            System.out.println("Log interaction 2: " + logResult2);

            // Get topology
            String topology = proxy.getTopologyMap();
            System.out.println("Current topology: " + topology);

            // Clear audit log
            String clearResult = proxy.clearAuditLog();
            System.out.println("Clear audit log: " + clearResult);

            // Configuration Example
            System.out.println("\n=== Configuration Example ===");

            // Configure breaker
            String configBreaker = proxy.configureBreaker(10, 60); // 10 calls per 60 seconds
            System.out.println("Configure breaker: " + configBreaker);

            // Configure queue capacity
            String configQueue = proxy.configureQueueCapacity(2000);
            System.out.println("Configure queue capacity: " + configQueue);

        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
            e.printStackTrace();
        }

        System.out.println("\nHalt.rs Java example completed!");
    }
}
