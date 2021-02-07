use crate::utils::{is_type_diagnostic_item, snippet_with_applicability, span_lint_and_sugg};
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::LateContext;
use rustc_middle::ty;
use rustc_span::sym;

use super::BYTES_NTH;

pub(super) fn lints<'tcx>(cx: &LateContext<'tcx>, expr: &Expr<'_>, iter_args: &'tcx [Expr<'tcx>]) {
    if_chain! {
        if let ExprKind::MethodCall(_, _, ref args, _) = expr.kind;
        let caller_type = if is_type_diagnostic_item(cx, cx.typeck_results().expr_ty(&iter_args[0]), sym::string_type) {
            Some("String")
        } else if let ty::Ref(_, tam, _) = cx.typeck_results().expr_ty(&iter_args[0]).kind() {
            match tam.kind() {
                ty::Str => Some("str"),
                _ => None
            }
        } else {
            None
        };
        if let Some(caller_type) = caller_type;
        then {
            let mut applicability = Applicability::MachineApplicable;
            span_lint_and_sugg(
                cx,
                BYTES_NTH,
                expr.span,
                &format!("called `.byte().nth()` on a `{}`", caller_type),
                "try calling `.as_bytes().get()`",
                format!(
                    "{}.as_bytes().get({})",
                    snippet_with_applicability(cx, iter_args[0].span, "..", &mut applicability),
                    snippet_with_applicability(cx, args[1].span, "..", &mut applicability)
                ),
                applicability,
            );
        }
    }
}
