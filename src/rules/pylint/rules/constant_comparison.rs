use itertools::Itertools;
use rustpython_ast::{Cmpop, Expr, ExprKind, Located};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::define_violation;
use crate::registry::Diagnostic;
use crate::violation::Violation;
use ruff_macros::derive_message_formats;

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ViolationsCmpop {
    Eq,
    NotEq,
    Lt,
    LtE,
    Gt,
    GtE,
    Is,
    IsNot,
    In,
    NotIn,
}

impl From<&Cmpop> for ViolationsCmpop {
    fn from(cmpop: &Cmpop) -> Self {
        match cmpop {
            Cmpop::Eq => Self::Eq,
            Cmpop::NotEq => Self::NotEq,
            Cmpop::Lt => Self::Lt,
            Cmpop::LtE => Self::LtE,
            Cmpop::Gt => Self::Gt,
            Cmpop::GtE => Self::GtE,
            Cmpop::Is => Self::Is,
            Cmpop::IsNot => Self::IsNot,
            Cmpop::In => Self::In,
            Cmpop::NotIn => Self::NotIn,
        }
    }
}

impl fmt::Display for ViolationsCmpop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let representation = match self {
            Self::Eq => "==",
            Self::NotEq => "!=",
            Self::Lt => "<",
            Self::LtE => "<=",
            Self::Gt => ">",
            Self::GtE => ">=",
            Self::Is => "is",
            Self::IsNot => "is not",
            Self::In => "in",
            Self::NotIn => "not in",
        };
        write!(f, "{representation}")
    }
}

define_violation!(
    pub struct ConstantComparison {
        pub left_constant: String,
        pub op: ViolationsCmpop,
        pub right_constant: String,
    }
);
impl Violation for ConstantComparison {
    #[derive_message_formats]
    fn message(&self) -> String {
        let ConstantComparison {
            left_constant,
            op,
            right_constant,
        } = self;

        format!(
            "Two constants compared in a comparison, consider replacing `{left_constant} {op} \
             {right_constant}`"
        )
    }
}

/// PLR0133
pub fn constant_comparison(
    checker: &mut Checker,
    left: &Expr,
    ops: &[Cmpop],
    comparators: &[Expr],
) {
    for ((left, right), op) in std::iter::once(left)
        .chain(comparators.iter())
        .tuple_windows::<(&Located<_>, &Located<_>)>()
        .zip(ops)
    {
        if let (
            ExprKind::Constant {
                value: left_constant,
                ..
            },
            ExprKind::Constant {
                value: right_constant,
                ..
            },
        ) = (&left.node, &right.node)
        {
            let diagnostic = Diagnostic::new(
                ConstantComparison {
                    left_constant: left_constant.to_string(),
                    op: op.into(),
                    right_constant: right_constant.to_string(),
                },
                Range::from_located(left),
            );

            checker.diagnostics.push(diagnostic);
        };
    }
}
