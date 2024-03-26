//! Detect potential overflow scenarios where the number of bits being shifted exceeds the bit width of
//! the variable being shifted, which could lead to unintended behavior or loss of data. If such a
//! potential overflow is detected, a warning is generated to alert the developer.
use crate::{
    diag,
    diagnostics::{
        codes::{custom, DiagnosticInfo, Severity},
        WarningFilters,
    },
    shared::{program_info::TypingProgramInfo, CompilationEnv},
    typing::{
        ast::{self as T, SequenceItem_, UnannotatedExp_},
        visitor::{TypingVisitorConstructor, TypingVisitorContext},
    },
};
use move_ir_types::location::Loc;

use super::{LinterDiagCategory, LINTER_DEFAULT_DIAG_CODE, LINT_WARNING_PREFIX};

const EMPTY_ELSE_BRANCH_DIAG: DiagnosticInfo = custom(
    LINT_WARNING_PREFIX,
    Severity::Warning,
    LinterDiagCategory::EmptyElseBranch as u8,
    LINTER_DEFAULT_DIAG_CODE,
    "",
);

pub struct EmptyElseBranch;

pub struct Context<'a> {
    env: &'a mut CompilationEnv,
}

impl TypingVisitorConstructor for EmptyElseBranch {
    type Context<'a> = Context<'a>;

    fn context<'a>(
        env: &'a mut CompilationEnv,
        _program_info: &'a TypingProgramInfo,
        _program: &T::Program_,
    ) -> Self::Context<'a> {
        Context { env }
    }
}

impl TypingVisitorContext for Context<'_> {
    fn visit_exp_custom(&mut self, exp: &mut T::Exp) -> bool {
        if let UnannotatedExp_::IfElse(_, _, else_block) = &exp.exp.value {
            // Determine if the else block is empty
            let mut else_block_is_empty = false;
            if let UnannotatedExp_::Block(seq) = &else_block.exp.value {
                if seq.1.len() == 1 {
                    if let SequenceItem_::Seq(seq_exp) = &seq.1[0].value {
                        if matches!(seq_exp.exp.value, UnannotatedExp_::Unit { trailing: true }) {
                            else_block_is_empty = true;
                        }
                    }
                };
            }

            if else_block_is_empty {
                report_empty_else_branch(self.env, else_block.exp.loc);
            }
        }
        false
    }
    fn add_warning_filter_scope(&mut self, filter: WarningFilters) {
        self.env.add_warning_filter_scope(filter)
    }

    fn pop_warning_filter_scope(&mut self) {
        self.env.pop_warning_filter_scope()
    }
}

fn report_empty_else_branch(env: &mut CompilationEnv, loc: Loc) {
    let diag = diag!(
        EMPTY_ELSE_BRANCH_DIAG,
        (
            loc,
            "Detected an empty `else` branch, which may be unnecessary."
        )
    );
    env.add_diag(diag);
}
