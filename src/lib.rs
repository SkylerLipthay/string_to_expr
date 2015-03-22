#![feature(plugin_registrar, rustc_private)]

extern crate rustc;
extern crate syntax;

use syntax::ast;
use syntax::codemap;
use syntax::ext::base::{DummyResult, ExtCtxt, MacResult};
use syntax::fold::Folder;
use syntax::parse::{self, token};
use rustc::plugin::Registry;

mod parser_any_macro;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("string_to_expr", string_to_expr);
}

fn string_to_expr<'a>(cx: &'a mut ExtCtxt,
                      sp: codemap::Span,
                      tts: &[ast::TokenTree]) -> Box<MacResult + 'a> {
    use syntax::print::pprust;

    let mut parser = parse::new_parser_from_tts(cx.parse_sess(), cx.cfg(), tts.to_vec());
    let arg = cx.expander().fold_expr(parser.parse_expr());
    let expr_string = match arg.node {
        ast::ExprLit(ref spanned) => {
            match spanned.node {
                ast::LitStr(ref s, _) => s.to_string(),
                _ => {
                    cx.span_err(sp, &format!("expected string literal but got `{}`",
                            pprust::lit_to_string(&*spanned))[..]);
                    return DummyResult::expr(sp)
                }
            }
        }
        _ => {
            cx.span_err(sp, &format!("expected string literal but got `{}`",
                    pprust::expr_to_string(&*arg))[..]);
            return DummyResult::expr(sp)
        }
    };

    if !parser.eat(&token::Token::Eof) {
        cx.span_err(parser.span, "only one string literal allowed");
        return DummyResult::expr(sp);
    }

    let sess = cx.parse_sess();
    let cfg = cx.cfg();
    let name = "string_expr".to_string();
    let parser = parse::new_parser_from_source_str(sess, cfg, name, expr_string);
    Box::new(parser_any_macro::ParserAnyMacro::new(parser)) as Box<MacResult>
}
