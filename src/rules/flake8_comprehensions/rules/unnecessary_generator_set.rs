use super::helpers;
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::rules::flake8_comprehensions::fixes;
use crate::violation::AlwaysAutofixableViolation;
use log::error;
use ruff_macros::derive_message_formats;
use rustpython_ast::{Expr, ExprKind, Keyword};

define_violation!(
    pub struct UnnecessaryGeneratorSet;
);
impl AlwaysAutofixableViolation for UnnecessaryGeneratorSet {
    #[derive_message_formats]
    fn message(&self) -> String {
        format!("Unnecessary generator (rewrite as a `set` comprehension)")
    }

    fn autofix_title(&self) -> String {
        "Rewrite as a `set` comprehension".to_string()
    }
}

/// C401 (`set(generator)`)
pub fn unnecessary_generator_set(
    checker: &mut Checker,
    expr: &Expr,
    func: &Expr,
    args: &[Expr],
    keywords: &[Keyword],
) {
    let Some(argument) = helpers::exactly_one_argument_with_matching_function("set", func, args, keywords) else {
        return;
    };
    if !checker.is_builtin("set") {
        return;
    }
    if let ExprKind::GeneratorExp { .. } = argument {
        let mut diagnostic = Diagnostic::new(UnnecessaryGeneratorSet, Range::from_located(expr));
        if checker.patch(diagnostic.kind.rule()) {
            match fixes::fix_unnecessary_generator_set(checker.locator, checker.stylist, expr) {
                Ok(fix) => {
                    diagnostic.amend(fix);
                }
                Err(e) => error!("Failed to generate fix: {e}"),
            }
        }
        checker.diagnostics.push(diagnostic);
    }
}
