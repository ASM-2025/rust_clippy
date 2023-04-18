use super::HAS_LOOP;

use clippy_utils::diagnostics::span_lint;
use rustc_lint::LateContext;
use rustc_span::source_map::Span;

// TODO: Adjust the parameters as necessary
pub(super) fn check(cx: &LateContext<'_>, span: Span) {
    span_lint(cx, HAS_LOOP, span, "Loop detected");
}
