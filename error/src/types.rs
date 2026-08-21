//! Provides error types.
//! 
//! Unrecoverable errors carry a descriptive message, while recoverable errors do not. 
//! The rationale is that recoverable tipically don't need require additional context beyond their variant,
//!  whereas unrecoverable errors need to provide information about the cause of failure.
