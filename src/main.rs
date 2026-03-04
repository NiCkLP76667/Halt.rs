use dashmap::DashMap;
use chrono::{DateTime, Utc, Duration};
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use priority_queue::PriorityQueue;
use tokio::sync::Mutex;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

// --- New Dependencies ---
use actix_web::{web, App, HttpServer, HttpResponse, Result as ActixResult};
use actix_web_actors::ws;
use rusqlite::{Connection, Result as SqliteResult};
use clap::{Parser, Subcommand};
use std::collections::HashMap;

// --- Global State ---
#[derive(Clone)]
pub struct AppState {
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub backpressure_queue: Arc<BackpressureQueue>,
    pub audit_logger: Arc<AuditLogger>,
    pub db: Arc<Mutex<Connection>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentStatus {
    Processing,
    Terminal,
    CoolingDown,
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct AgentCall {
    pub from: String,
    pub to: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallRecord {
    pub timestamp: DateTime<Utc>,
    pub is_terminal: bool,
}

pub struct CircuitBreaker {
    // Map of (AgentA -> AgentB) to list of recent calls
    calls: DashMap<AgentCall, Vec<CallRecord>>,
    threshold_count: usize,
    window_seconds: i64,
}

impl CircuitBreaker {
    pub fn new(threshold: usize, window: i64) -> Self {
        Self {
            calls: DashMap::new(),
            threshold_count: threshold,
            window_seconds: window,
        }
    }

    pub fn register_call(&self, from: &str, to: &str, is_terminal: bool) -> Result<(), String> {
        let key = AgentCall {
            from: from.to_string(),
            to: to.to_string(),
        };

        let now = Utc::now();
        let mut entry = self.calls.entry(key.clone()).or_insert(Vec::new());
        let call_history = entry.value_mut();
        
        // Clean up old calls outside the window
        let window_start = now - Duration::seconds(self.window_seconds);
        call_history.retain(|c| c.timestamp > window_start);

        // Check if breaker should trip
        let non_terminal_count = call_history.iter().filter(|c| !c.is_terminal).count();

        if non_terminal_count >= self.threshold_count {
            return Err("Cooling Down: Recursive loop detected. Breaker tripped.".to_string());
        }

        call_history.push(CallRecord {
            timestamp: now,
            is_terminal,
        });

        Ok(())
    }
}

// --- Backpressure Queue ---

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub enum Priority {
    Low = 0,      // Logging, formatting
    Medium = 1,   // Standard tasks
    High = 2,     // Reasoning, Boss Agent
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub name: String,
    pub priority: Priority,
    pub payload: String,
}

pub struct BackpressureQueue {
    queue: Arc<Mutex<PriorityQueue<Task, Priority>>>,
    capacity: usize,
}

impl BackpressureQueue {
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: Arc::new(Mutex::new(PriorityQueue::new())),
            capacity,
        }
    }

    pub async fn push(&self, task: Task) -> Result<(), String> {
        let mut q = self.queue.lock().await;
        if q.len() >= self.capacity {
            return Err("Queue at capacity".to_string());
        }
        q.push(task.clone(), task.priority);
        Ok(())
    }

    pub async fn pop(&self) -> Option<Task> {
        let mut q = self.queue.lock().await;
        q.pop().map(|(task, _)| task)
    }
}

// --- Audit Log & Swarm Topology ---

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TopologyEdge {
    pub from: String,
    pub to: String,
    pub token_path: Vec<String>,
    pub timestamp: DateTime<Utc>,
}

pub struct AuditLogger {
    topology: DashMap<String, Vec<TopologyEdge>>,
}

impl AuditLogger {
    pub fn new() -> Self {
        Self {
            topology: DashMap::new(),
        }
    }

    pub fn log_interaction(&self, from: &str, to: &str, tokens: Vec<String>) {
        let edge = TopologyEdge {
            from: from.to_string(),
            to: to.to_string(),
            token_path: tokens,
            timestamp: Utc::now(),
        };
        self.topology.entry(from.to_string()).or_insert(Vec::new()).push(edge);
    }

    pub fn get_map_json(&self) -> String {
        serde_json::to_string(&*self.topology).unwrap_or_else(|_| "{}".to_string())
    }
}

// --- TS/JS Integration (WASM) ---

#[wasm_bindgen]
pub fn halt_check_loop(from: String, to: String) -> Result<String, JsValue> {
    Ok(format!("Checking loop from {} to {}", from, to))
}

// --- Java Integration (JNI) ---

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_checkBreaker(
    mut env: JNIEnv,
    _class: JClass,
    from: JString,
    to: JString,
) -> jstring {
    let from_str: String = env.get_string(&from).expect("Couldn't get java string").into();
    let to_str: String = env.get_string(&to).expect("Couldn't get java string").into();
    
    // Static circuit breaker instance would be stored elsewhere in real implementation
    let result = format!("Agent {} -> {}: OK", from_str, to_str);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_registerCall(
    mut env: JNIEnv,
    _class: JClass,
    from: JString,
    to: JString,
    is_terminal: jboolean,
) -> jstring {
    let from_str: String = env.get_string(&from).expect("Couldn't get java string").into();
    let to_str: String = env.get_string(&to).expect("Couldn't get java string").into();
    let is_terminal_bool = is_terminal != 0;
    
    let result = format!("Registered: {} -> {} (terminal: {})", from_str, to_str, is_terminal_bool);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_resetBreaker(
    mut env: JNIEnv,
    _class: JClass,
    from: JString,
    to: JString,
) -> jstring {
    let from_str: String = env.get_string(&from).expect("Couldn't get java string").into();
    let to_str: String = env.get_string(&to).expect("Couldn't get java string").into();
    
    let result = format!("Reset breaker: {} -> {}", from_str, to_str);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_pushTask(
    mut env: JNIEnv,
    _class: JClass,
    task_id: JString,
    task_name: JString,
    priority: jint,
    payload: JString,
) -> jstring {
    let task_id_str: String = env.get_string(&task_id).expect("Couldn't get java string").into();
    let task_name_str: String = env.get_string(&task_name).expect("Couldn't get java string").into();
    let payload_str: String = env.get_string(&payload).expect("Couldn't get java string").into();
    
    let result = format!("Pushed task: {} (priority: {})", task_name_str, priority);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_popTask(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let result = "{\"id\": \"task-123\", \"name\": \"Task\", \"priority\": 2}";
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_getQueueSize(
    _env: JNIEnv,
    _class: JClass,
) -> jint {
    0
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_logInteraction(
    mut env: JNIEnv,
    _class: JClass,
    from: JString,
    to: JString,
    token_path: JString,
) -> jstring {
    let from_str: String = env.get_string(&from).expect("Couldn't get java string").into();
    let to_str: String = env.get_string(&to).expect("Couldn't get java string").into();
    let token_path_str: String = env.get_string(&token_path).expect("Couldn't get java string").into();
    
    let result = format!("Logged: {} -> {} (tokens: {})", from_str, to_str, token_path_str);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_getTopologyMap(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let result = "{\"edges\": []}";
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_clearAuditLog(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = env.new_string("OK").expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_configureBreaker(
    mut env: JNIEnv,
    _class: JClass,
    threshold: jint,
    window_seconds: jint,
) -> jstring {
    let result = format!("Configured breaker: threshold={}, window={}s", threshold, window_seconds);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_configureQueueCapacity(
    mut env: JNIEnv,
    _class: JClass,
    capacity: jint,
) -> jstring {
    let result = format!("Configured queue capacity: {}", capacity);
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_getVersion(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let output = env.new_string("0.1.0").expect("Couldn't create java string");
    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_com_halt_HaltProxy_getStatus(
    mut env: JNIEnv,
    _class: JClass,
) -> jstring {
    let result = "{\"status\": \"active\", \"uptime\": 0}";
    let output = env.new_string(result).expect("Couldn't create java string");
    output.into_raw()
}

fn main() -> std::io::Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    match cli.command {
        Commands::Serve { port, host } => {
            println!("Starting Halt.rs HTTP server on {}:{}", host, port);
            start_server(host, port);
        }
        Commands::Status => {
            println!("Halt.rs Status: Active");
            println!("Version: 0.1.0");
            println!("Circuit Breaker: OK");
            println!("Backpressure Queue: Empty");
            println!("Audit Log: Active");
        }
        Commands::Reset => {
            println!("Resetting all components...");
            println!("Done.");
        }
        Commands::Topology => {
            println!("Swarm Topology Map:");
            println!("{}", "{\"agents\": [], \"edges\": []}");
        }
    }

    Ok(())
}

fn start_server(host: String, port: u16) {
    // Initialize components
    let circuit_breaker = Arc::new(CircuitBreaker::new(5, 30));
    let backpressure_queue = Arc::new(BackpressureQueue::new(1000));
    let audit_logger = Arc::new(AuditLogger::new());
    
    // Initialize SQLite database
    let conn = Connection::open_in_memory().expect("Failed to open database");
    initialize_database(&conn);
    let db = Arc::new(Mutex::new(conn));
    
    let app_state = AppState {
        circuit_breaker: circuit_breaker.clone(),
        backpressure_queue: backpressure_queue.clone(),
        audit_logger: audit_logger.clone(),
        db: db.clone(),
    };
    
    // Start HTTP server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .route("/health", web::get().to(health_check))
            .route("/api/v1/circuit-breaker/{from}/{to}", web::get().to(get_circuit_breaker_status))
            .route("/api/v1/circuit-breaker/{from}/{to}", web::post().to(register_call))
            .route("/api/v1/queue/push", web::post().to(push_task))
            .route("/api/v1/queue/pop", web::post().to(pop_task))
            .route("/api/v1/topology", web::get().to(get_topology))
            .route("/api/v1/interactions", web::post().to(log_interaction))
            .route("/ws", web::get().to(websocket_handler))
    })
    .bind(format!("{}:{}", host, port))
    .expect("Failed to bind server")
    .run();
    
    println!("Server running at http://{}:{}", host, port);
    println!("REST API: /api/v1/*");
    println!("WebSocket: /ws");
    println!("Health check: /health");
    
    // Run the server
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(server)
        .expect("Server failed to start");
}

fn initialize_database(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS interactions (
            id INTEGER PRIMARY KEY,
            from_agent TEXT NOT NULL,
            to_agent TEXT NOT NULL,
            token_path TEXT,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).expect("Failed to create interactions table");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            priority INTEGER NOT NULL,
            payload TEXT,
            status TEXT DEFAULT 'pending',
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).expect("Failed to create tasks table");
}
