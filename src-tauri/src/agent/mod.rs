//! AI Agent 集成模块
//!
//! 提供基于 OpenAI 兼容 API 的 Agent 实现

pub mod native_agent;
pub mod types;

pub use native_agent::{NativeAgent, NativeAgentState};
pub use types::*;
