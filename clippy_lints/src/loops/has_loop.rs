use super::HAS_LOOP;

use clippy_utils::diagnostics::span_lint;
use rustc_hir::Expr;
use rustc_hir::HirId;
use rustc_hir::ItemKind;
use rustc_hir::Node;
use rustc_lint::LateContext;

fn get_enclosing_func(cx: &LateContext<'_>, mut hir_id: HirId) -> Option<HirId> {
    let hir = cx.tcx.hir();

    loop {
        if let Some(node) = hir.find(hir_id) {
            // dbg!(node);
            if let Node::Item(item) = node {
                if let ItemKind::Fn(_, _, _) = item.kind {
                    // dbg!(item.ident.name);
                    return Some(hir_id);
                }
            }
        }
        hir_id = hir.parent_id(hir_id);
    }
}

pub(super) fn check(cx: &LateContext<'_>, expr: &Expr<'_>) {
    let hir = cx.tcx.hir();
    let hir_id = expr.hir_id;
    let hir_id = get_enclosing_func(cx, hir_id).unwrap();

    let mut fn_name: &str = "UNKNWON";

    if let Some(Node::Item(item)) = hir.find(hir_id) {
        if let ItemKind::Fn(_, _, _) = item.kind {
            fn_name = item.ident.name.as_str();
        }
    }

    span_lint(
        cx,
        HAS_LOOP,
        expr.span,
        format!("Loop detected in function \"{}\"", fn_name).as_str(),
    );
}
