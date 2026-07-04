//! Boa JS 引擎封装
//!
//! 使用 boa_ast-0.21.1, boa_engine-0.21.1, boa_gc-0.21.1, boa_interner-0.21.1, boa_parser-0.21.1, boa_string-0.21.1 作为 Rust 原生 JS 解释器。
//! 不依赖系统 WebView 的 V8，所有书源规则在 Rust 层执行。
//!
//! 相关字符串从 .so 提取:
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\expression\access.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\expression\literal\object.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\expression\operator\assign\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\expression\operator\binary\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\expression\operator\conditional.r
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\function\class.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\scope.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\scope_analyzer.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_ast-0.21.1\src\source_text.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\bigint.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\array\array_iterator.
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\array\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\array_buffer\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\array_buffer\shared.r
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\array_buffer\utils.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\async_function\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\async_generator\mod.r
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\async_generator_funct
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\atomics\futex.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\atomics\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\bigint\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\boolean\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\builder.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\dataview\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\date\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\date\utils.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\error\aggregate.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\error\eval.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\error\mod.rs
//! - C:\Users\q3499\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\boa_engine-0.21.1\src\builtins\error\range.rs

use boa_engine::{Context, JsValue, JsResult, Source, NativeFunction};
use std::sync::Mutex;
use parking_lot::RwLock;

/// Boa 引擎池 (可复用 Context)
pub struct EnginePool {
    pool: Mutex<Vec<Context>>,
    max_size: usize,
}

impl EnginePool {
    pub fn new(max_size: usize) -> Self {
        Self { pool: Mutex::new(Vec::new()), max_size }
    }

    pub fn get(&self) -> EngineGuard {
        let ctx = self.pool.lock().unwrap().pop().unwrap_or_else(|| Context::default());
        EngineGuard { pool: self, ctx: Some(ctx) }
    }
}

pub struct EngineGuard<'a> {
    pool: &'a EnginePool,
    ctx: Option<Context>,
}

impl<'a> Drop for EngineGuard<'a> {
    fn drop(&mut self) {
        if let Some(ctx) = self.ctx.take() {
            let mut pool = self.pool.pool.lock().unwrap();
            if pool.len() < self.pool.max_size {
                pool.push(ctx);
            }
        }
    }
}

/// 执行 JS 代码 (对应命令: js_eval)
pub fn js_eval(code: &str) -> anyhow::Result<String> {
    let mut ctx = Context::default();
    let result = ctx.eval(Source::from_bytes(code))?;
    Ok(result.to_string(&mut ctx)?.to_std_string_escaped())
}

/// 执行书源规则
pub fn eval_booksource(rule: &str, html: &str) -> anyhow::Result<String> {
    let mut ctx = Context::default();
    // 注入 HTML 和工具函数到 Boa 上下文
    let html_var = format!("{}", html);  // 简化: 实际用 JsValue::string
    ctx.eval(Source::from_bytes(&format!(
        "var html = {}; var result = (function() { {} })();", rule
    )))?;
    let result = ctx.eval(Source::from_bytes("result"))?;
    Ok(result.to_string(&mut ctx)?.to_std_string_escaped())
}
