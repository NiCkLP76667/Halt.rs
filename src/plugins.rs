// Halt.rs Plugin System
// Extensible architecture for custom circuit breaker strategies,
// queue implementations, and audit loggers

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Plugin metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub capabilities: Vec<String>,
}

/// Plugin trait for circuit breaker strategies
#[async_trait]
pub trait CircuitBreakerPlugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;

    /// Check if a call should be allowed
    async fn check_call(&self, from: &str, to: &str) -> Result<(), String>;

    /// Register a call (may trip the breaker)
    async fn register_call(&self, from: &str, to: &str, is_terminal: bool) -> Result<(), String>;

    /// Reset the breaker for a pair
    async fn reset(&self, from: &str, to: &str) -> Result<(), String>;

    /// Get breaker statistics
    async fn stats(&self) -> HashMap<String, serde_json::Value>;
}

/// Plugin trait for queue implementations
#[async_trait]
pub trait QueuePlugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;

    /// Push a task to the queue
    async fn push(&self, task: Task) -> Result<(), String>;

    /// Pop the highest priority task
    async fn pop(&self) -> Option<Task>;

    /// Get queue statistics
    async fn stats(&self) -> HashMap<String, serde_json::Value>;

    /// Get current queue size
    async fn size(&self) -> usize;
}

/// Plugin trait for audit loggers
#[async_trait]
pub trait AuditPlugin: Send + Sync {
    fn metadata(&self) -> PluginMetadata;

    /// Log an interaction
    async fn log_interaction(&self, from: &str, to: &str, tokens: Vec<String>) -> Result<(), String>;

    /// Get topology map
    async fn get_topology(&self) -> Result<String, String>;

    /// Get audit statistics
    async fn stats(&self) -> HashMap<String, serde_json::Value>;
}

/// Plugin manager for loading and managing plugins
pub struct PluginManager {
    circuit_breakers: HashMap<String, Arc<dyn CircuitBreakerPlugin>>,
    queues: HashMap<String, Arc<dyn QueuePlugin>>,
    auditors: HashMap<String, Arc<dyn AuditPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            circuit_breakers: HashMap::new(),
            queues: HashMap::new(),
            auditors: HashMap::new(),
        }
    }

    /// Register a circuit breaker plugin
    pub fn register_circuit_breaker(&mut self, plugin: Arc<dyn CircuitBreakerPlugin>) {
        let name = plugin.metadata().name;
        self.circuit_breakers.insert(name, plugin);
    }

    /// Register a queue plugin
    pub fn register_queue(&mut self, plugin: Arc<dyn QueuePlugin>) {
        let name = plugin.metadata().name;
        self.queues.insert(name, plugin);
    }

    /// Register an audit plugin
    pub fn register_auditor(&mut self, plugin: Arc<dyn AuditPlugin>) {
        let name = plugin.metadata().name;
        self.auditors.insert(name, plugin);
    }

    /// Get circuit breaker by name
    pub fn get_circuit_breaker(&self, name: &str) -> Option<Arc<dyn CircuitBreakerPlugin>> {
        self.circuit_breakers.get(name).cloned()
    }

    /// Get queue by name
    pub fn get_queue(&self, name: &str) -> Option<Arc<dyn QueuePlugin>> {
        self.queues.get(name).cloned()
    }

    /// Get auditor by name
    pub fn get_auditor(&self, name: &str) -> Option<Arc<dyn AuditPlugin>> {
        self.auditors.get(name).cloned()
    }

    /// List all registered plugins
    pub fn list_plugins(&self) -> HashMap<String, Vec<PluginMetadata>> {
        let mut result = HashMap::new();

        result.insert("circuit_breakers".to_string(),
            self.circuit_breakers.values().map(|p| p.metadata()).collect());

        result.insert("queues".to_string(),
            self.queues.values().map(|p| p.metadata()).collect());

        result.insert("auditors".to_string(),
            self.auditors.values().map(|p| p.metadata()).collect());

        result
    }
}

/// Example: Custom Circuit Breaker Plugin
pub struct ExponentialBackoffBreaker {
    metadata: PluginMetadata,
    call_counts: Arc<RwLock<HashMap<String, usize>>>,
    last_reset: Arc<RwLock<HashMap<String, chrono::DateTime<chrono::Utc>>>>,
}

impl ExponentialBackoffBreaker {
    pub fn new() -> Self {
        Self {
            metadata: PluginMetadata {
                name: "exponential_backoff".to_string(),
                version: "1.0.0".to_string(),
                description: "Circuit breaker with exponential backoff".to_string(),
                author: "Halt.rs Team".to_string(),
                capabilities: vec!["circuit_breaker".to_string()],
            },
            call_counts: Arc::new(RwLock::new(HashMap::new())),
            last_reset: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl CircuitBreakerPlugin for ExponentialBackoffBreaker {
    fn metadata(&self) -> PluginMetadata {
        self.metadata.clone()
    }

    async fn check_call(&self, from: &str, to: &str) -> Result<(), String> {
        let key = format!("{}->{}", from, to);
        let call_counts = self.call_counts.read().await;
        let count = call_counts.get(&key).unwrap_or(&0);

        // Exponential backoff logic
        if *count > 0 {
            let backoff_duration = std::time::Duration::from_secs(2u64.pow(*count as u32));
            if let Some(last_reset) = self.last_reset.read().await.get(&key) {
                if chrono::Utc::now() < *last_reset + chrono::Duration::from_std(backoff_duration).unwrap() {
                    return Err("Circuit breaker tripped - exponential backoff active".to_string());
                }
            }
        }

        Ok(())
    }

    async fn register_call(&self, from: &str, to: &str, is_terminal: bool) -> Result<(), String> {
        let key = format!("{}->{}", from, to);

        if !is_terminal {
            let mut call_counts = self.call_counts.write().await;
            let count = call_counts.entry(key.clone()).or_insert(0);
            *count += 1;

            // Check if we should trip
            if *count >= 5 {  // Threshold
                let mut last_reset = self.last_reset.write().await;
                last_reset.insert(key, chrono::Utc::now());
                return Err("Circuit breaker tripped".to_string());
            }
        } else {
            // Reset on terminal state
            let mut call_counts = self.call_counts.write().await;
            call_counts.remove(&key);
            let mut last_reset = self.last_reset.write().await;
            last_reset.remove(&key);
        }

        Ok(())
    }

    async fn reset(&self, from: &str, to: &str) -> Result<(), String> {
        let key = format!("{}->{}", from, to);
        let mut call_counts = self.call_counts.write().await;
        call_counts.remove(&key);
        let mut last_reset = self.last_reset.write().await;
        last_reset.remove(&key);
        Ok(())
    }

    async fn stats(&self) -> HashMap<String, serde_json::Value> {
        let mut stats = HashMap::new();
        let call_counts = self.call_counts.read().await;
        stats.insert("active_breakers".to_string(), serde_json::json!(call_counts.len()));
        stats.insert("call_counts".to_string(), serde_json::json!(call_counts.clone()));
        stats
    }
}

/// Plugin loader for dynamic loading
pub struct PluginLoader;

impl PluginLoader {
    pub fn load_from_directory(dir: &std::path::Path) -> Result<PluginManager, String> {
        let mut manager = PluginManager::new();

        // In a real implementation, this would scan the directory for
        // plugin libraries (.so, .dll, .dylib) and load them dynamically

        // For now, register built-in plugins
        manager.register_circuit_breaker(Arc::new(ExponentialBackoffBreaker::new()));

        Ok(manager)
    }
}

/// Plugin configuration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled_plugins: Vec<String>,
    pub plugin_directory: Option<String>,
    pub plugin_settings: HashMap<String, serde_json::Value>,
}

impl Default for PluginConfig {
    fn default() -> Self {
        Self {
            enabled_plugins: vec!["exponential_backoff".to_string()],
            plugin_directory: Some("./plugins".to_string()),
            plugin_settings: HashMap::new(),
        }
    }
}
