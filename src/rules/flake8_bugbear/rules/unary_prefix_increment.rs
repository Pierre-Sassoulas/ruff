//! Checks for `++n`.
//!
//! ## Why is this bad?
//!
//! Python does not support the unary prefix increment. Writing `++n` is
//! equivalent to `+(+(n))`, which equals `n`.
//!
//! ## Example
//!
//! ```python
//! ++n;
//! ```
//!
//! Use instead:
//!
//! ```python
//! n += 1
//! ```

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;
use ruff_macros::derive_message_formats;
use rustpython_ast::{Expr, ExprKind, Unaryop};

define_violation!(
    pub struct UnaryPrefixIncrement;
);
impl Violation for UnaryPrefixIncrement {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Python does not support the unary prefix increment")
    }
}

/// B002
pub fn unary_prefix_increment(checker: &mut Checker, expr: &Expr, op: &Unaryop, operand: &Expr) {
    if !matches!(op, Unaryop::UAdd) {
        return;
    }
    let ExprKind::UnaryOp { op, .. } = &operand.node else {
            return;
        };
    if !matches!(op, Unaryop::UAdd) {
        return;
    }
    checker.diagnostics.push(Diagnostic::new(
        UnaryPrefixIncrement,
        Range::from_located(expr),
    ));
}
