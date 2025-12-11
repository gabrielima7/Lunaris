//! Scripting benchmarks

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lunaris_scripting::{SandboxConfig, ScriptEngine};

fn script_creation_benchmark(c: &mut Criterion) {
    c.bench_function("script_engine_creation", |b| {
        b.iter(|| {
            let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();
            black_box(engine);
        });
    });
}

fn simple_script_benchmark(c: &mut Criterion) {
    let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();

    c.bench_function("simple_script_exec", |b| {
        b.iter(|| {
            engine.run_script(black_box("local x = 1 + 1")).unwrap();
        });
    });
}

fn math_script_benchmark(c: &mut Criterion) {
    let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();

    c.bench_function("math_heavy_script", |b| {
        b.iter(|| {
            let result: f64 = engine
                .eval(black_box(
                    r#"
                    local sum = 0
                    for i = 1, 100 do
                        sum = sum + math.sin(i) * math.cos(i)
                    end
                    return sum
                "#,
                ))
                .unwrap();
            black_box(result);
        });
    });
}

fn lunaris_api_benchmark(c: &mut Criterion) {
    let engine = ScriptEngine::new(SandboxConfig::default()).unwrap();

    c.bench_function("lunaris_api_calls", |b| {
        b.iter(|| {
            let result: f64 = engine
                .eval(black_box(
                    r#"
                    local result = 0
                    for i = 1, 100 do
                        result = lunaris.math.lerp(0, 100, i / 100)
                    end
                    return result
                "#,
                ))
                .unwrap();
            black_box(result);
        });
    });
}

criterion_group!(
    benches,
    script_creation_benchmark,
    simple_script_benchmark,
    math_script_benchmark,
    lunaris_api_benchmark
);

criterion_main!(benches);
