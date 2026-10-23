use ruff_formatter::prelude::{space, token};
use ruff_formatter::{write, FormatOptions};
use ruff_python_ast::helpers::any_over_expr;
use ruff_python_ast::{Expr, StmtAssert};
use ruff_python_parser::TokenKind;
use ruff_text_size::Ranged;

use crate::comments::SourceComment;

use crate::expression::maybe_parenthesize_expression;
use crate::expression::parentheses::Parenthesize;
use crate::{has_skip_comment, prelude::*, MagicTrailingComma};

#[derive(Default)]
pub struct FormatStmtAssert;

impl FormatNodeRule<StmtAssert> for FormatStmtAssert {
    fn fmt_fields(&self, item: &StmtAssert, f: &mut PyFormatter) -> FormatResult<()> {
        // unsloth: a multi-line assert with no comment whose lines, joined with single
        // spaces, fit the line width ignores its magic trailing commas so it collapses.
        let previous = collapses(item, f)
            .then(|| f.context_mut().replace_magic_trailing_comma(MagicTrailingComma::Ignore));
        let result = fmt_assert(&item.test, item.msg.as_deref(), item, f);
        if let Some(previous) = previous {
            f.context_mut().replace_magic_trailing_comma(previous);
        }
        result
    }

    fn is_suppressed(
        &self,
        trailing_comments: &[SourceComment],
        context: &PyFormatContext,
    ) -> bool {
        has_skip_comment(trailing_comments, context.source())
    }
}

fn collapses(item: &StmtAssert, f: &PyFormatter) -> bool {
    let source = f.context().source();
    let (lo, hi) = (item.start().to_usize(), item.end().to_usize());
    let start = source[..lo].rfind('\n').map_or(0, |i| i + 1);
    let end = source[hi..].find('\n').map_or(source.len(), |i| hi + i);
    let segment = &source[start..end];
    if !segment.contains('\n') {
        return false;
    }
    let first = segment.lines().next().unwrap_or("");
    let indent = first.len() - first.trim_start().len();
    let rows = segment.lines().count();
    let joined = segment.lines().map(|l| l.trim().chars().count()).sum::<usize>() + rows - 1;
    // A one-element tuple's comma is not magic; the hook leaves such an assert alone.
    let one_tuple = |e: &Expr| matches!(e, Expr::Tuple(t) if t.elts.len() == 1);
    if any_over_expr(&item.test, &one_tuple)
        || item.msg.as_deref().is_some_and(|m| any_over_expr(m, &one_tuple))
    {
        return false;
    }
    // The hook looks for a comment on every row the assert touches, so a trailing one counts.
    indent + joined <= f.options().line_width().value() as usize
        && !source[hi..end].contains('#')
        && !f
            .context()
            .tokens()
            .in_range(item.range())
            .iter()
            .any(|t| t.kind() == TokenKind::Comment)
}

fn fmt_assert(
    test: &Expr,
    msg: Option<&Expr>,
    item: &StmtAssert,
    f: &mut PyFormatter,
) -> FormatResult<()> {
    write!(
        f,
        [
            token("assert"),
            space(),
            maybe_parenthesize_expression(test, item, Parenthesize::IfBreaks)
        ]
    )?;

    if let Some(msg) = msg {
        write!(
            f,
            [
                token(","),
                space(),
                maybe_parenthesize_expression(msg, item, Parenthesize::IfBreaks),
            ]
        )?;
    }

    Ok(())
}
