use criterion::{black_box, criterion_group, criterion_main, Criterion};
use halt::{CircuitBreaker, BackpressureQueue, AuditLogger, Priority, Task};
use uuid::Uuid;

fn circuit_breaker_benchmark(c: &mut Criterion) {
    let breaker = CircuitBreaker::new(5, 30);

    c.bench_function("circuit_breaker_register_call", |b| {
        b.iter(|| {
            black_box(breaker.register_call("AgentA", "AgentB", false));
        });
    });

    c.bench_function("circuit_breaker_check_status", |b| {
        b.iter(|| {
            black_box(breaker.register_call("AgentA", "AgentB", false));
        });
    });
}

fn backpressure_queue_benchmark(c: &mut Criterion) {
    let queue = BackpressureQueue::new(10000);

    c.bench_function("backpressure_queue_push_high_priority", |b| {
        b.iter(|| {
            let task = Task {
                id: Uuid::new_v4(),
                name: "High Priority Task".to_string(),
                priority: Priority::High,
                payload: r#"{"type": "reasoning"}"#.to_string(),
            };
            black_box(queue.push(task));
        });
    });

    c.bench_function("backpressure_queue_pop", |b| {
        // Pre-populate queue
        for _ in 0..100 {
            let task = Task {
                id: Uuid::new_v4(),
                name: "Task".to_string(),
                priority: Priority::Medium,
                payload: r#"{"type": "task"}"#.to_string(),
            };
            let _ = queue.push(task);
        }

        b.iter(|| {
            black_box(queue.pop());
        });
    });
}

fn audit_logger_benchmark(c: &mut Criterion) {
    let logger = AuditLogger::new();

    c.bench_function("audit_logger_log_interaction", |b| {
        b.iter(|| {
            black_box(logger.log_interaction(
                "AgentA",
                "AgentB",
                vec!["token1".to_string(), "token2".to_string()]
            ));
        });
    });

    c.bench_function("audit_logger_get_topology", |b| {
        // Pre-populate with some data
        for i in 0..100 {
            logger.log_interaction(
                &format!("Agent{}", i),
                &format!("Agent{}", i + 1),
                vec![format!("token{}", i)]
            );
        }

        b.iter(|| {
            black_box(logger.get_map_json());
        });
    });
}

fn memory_usage_benchmark(c: &mut Criterion) {
    c.bench_function("memory_usage_large_topology", |b| {
        let logger = AuditLogger::new();

        // Create a large topology
        for i in 0..1000 {
            for j in 0..10 {
                logger.log_interaction(
                    &format!("Agent{}", i),
                    &format!("Agent{}", j),
                    vec![format!("token{}-{}", i, j)]
                );
            }
        }

        black_box(logger.get_map_json());
    });
}

fn concurrent_operations_benchmark(c: &mut Criterion) {
    let breaker = CircuitBreaker::new(100, 60);
    let queue = BackpressureQueue::new(10000);
    let logger = AuditLogger::new();

    c.bench_function("concurrent_mixed_operations", |b| {
        b.iter(|| {
            // Simulate concurrent operations
            let _ = breaker.register_call("AgentA", "AgentB", false);

            let task = Task {
                id: Uuid::new_v4(),
                name: "Concurrent Task".to_string(),
                priority: Priority::Medium,
                payload: r#"{"concurrent": true}"#.to_string(),
            };
            let _ = queue.push(task);

            logger.log_interaction("AgentA", "AgentB", vec!["concurrent_token".to_string()]);
        });
    });
}

criterion_group!(
    benches,
    circuit_breaker_benchmark,
    backpressure_queue_benchmark,
    audit_logger_benchmark,
    memory_usage_benchmark,
    concurrent_operations_benchmark
);
criterion_main!(benches);
