package com.halt;

/**
 * HaltProxy - Java interface for the Halt.rs multi-agent proxy system.
 * Provides circuit breaking, backpressure queuing, and audit logging capabilities.
 */
public class HaltProxy {
    static {
        try {
            System.loadLibrary("halt");
        } catch (UnsatisfiedLinkError e) {
            System.err.println("WARNING: Native library 'halt' not found. Make sure to build the Rust library first.");
        }
    }

    // --- Circuit Breaker Methods ---

    /**
     * Check if the circuit breaker should trip for a given agent pair.
     * @param from Source agent identifier
     * @param to Destination agent identifier
     * @return "OK" if safe, or "Cooling Down" error message if breaker is tripped
     */
    public native String checkBreaker(String from, String to);

    /**
     * Register a call between agents with the circuit breaker.
     * @param from Source agent identifier
     * @param to Destination agent identifier
     * @param isTerminal Whether the call reached a terminal state
     * @return "OK" if registered, or error message if breaker is tripped
     */
    public native String registerCall(String from, String to, boolean isTerminal);

    /**
     * Reset the circuit breaker for a specific agent pair.
     * @param from Source agent identifier
     * @param to Destination agent identifier
     * @return "OK" if reset successfully
     */
    public native String resetBreaker(String from, String to);

    // --- Backpressure Queue Methods ---

    /**
     * Push a task to the backpressure queue.
     * @param taskId Unique identifier for the task
     * @param taskName Name/description of the task
     * @param priority Priority level: 0=Low, 1=Medium, 2=High
     * @param payload Task payload data
     * @return "OK" if enqueued, or error message if queue is at capacity
     */
    public native String pushTask(String taskId, String taskName, int priority, String payload);

    /**
     * Pop the highest priority task from the queue.
     * @return JSON string containing task details, or "null" if queue is empty
     */
    public native String popTask();

    /**
     * Get the current queue size.
     * @return Number of tasks currently in queue
     */
    public native int getQueueSize();

    // --- Audit Log & Topology Methods ---

    /**
     * Log an interaction between agents for topology tracking.
     * @param from Source agent identifier
     * @param to Destination agent identifier
     * @param tokenPath JSON array of token identifiers in the call chain
     * @return "OK" if logged successfully
     */
    public native String logInteraction(String from, String to, String tokenPath);

    /**
     * Get the current swarm topology map as JSON.
     * @return JSON string representing the agent interaction graph
     */
    public native String getTopologyMap();

    /**
     * Clear the audit log and topology map.
     * @return "OK" if cleared successfully
     */
    public native String clearAuditLog();

    // --- Configuration Methods ---

    /**
     * Set circuit breaker threshold parameters.
     * @param threshold Maximum non-terminal calls before tripping
     * @param windowSeconds Time window in seconds for call counting
     * @return "OK" if configured successfully
     */
    public native String configureBreaker(int threshold, int windowSeconds);

    /**
     * Set backpressure queue capacity.
     * @param capacity Maximum number of tasks in queue
     * @return "OK" if configured successfully
     */
    public native String configureQueueCapacity(int capacity);

    // --- Utility Methods ---

    /**
     * Get the Halt proxy version information.
     * @return Version string
     */
    public native String getVersion();

    /**
     * Get current system status.
     * @return JSON string with system status information
     */
    public native String getStatus();

    public static void main(String[] args) {
        HaltProxy proxy = new HaltProxy();
        
        System.out.println("=== Halt.rs Multi-Agent Proxy ===");
        System.out.println("Version: " + proxy.getVersion());
        System.out.println("Status: " + proxy.getStatus());
        System.out.println();

        // Test circuit breaker
        System.out.println("--- Circuit Breaker Test ---");
        String status = proxy.checkBreaker("AgentA", "AgentB");
        System.out.println("AgentA -> AgentB: " + status);
        
        // Register a call
        String regResult = proxy.registerCall("AgentA", "AgentB", false);
        System.out.println("Register call result: " + regResult);
        System.out.println();

        // Test backpressure queue
        System.out.println("--- Backpressure Queue Test ---");
        String taskId = java.util.UUID.randomUUID().toString();
        String pushResult = proxy.pushTask(taskId, "Reasoning Task", 2, "{\"type\": \"reasoning\"}");
        System.out.println("Push task result: " + pushResult);
        System.out.println("Queue size: " + proxy.getQueueSize());
        
        String poppedTask = proxy.popTask();
        System.out.println("Popped task: " + poppedTask);
        System.out.println();

        // Test audit logging
        System.out.println("--- Audit Log Test ---");
        String logResult = proxy.logInteraction("AgentA", "AgentB", "[\"token1\", \"token2\"]");
        System.out.println("Log interaction result: " + logResult);
        
        String topology = proxy.getTopologyMap();
        System.out.println("Current topology: " + topology);
    }
}
