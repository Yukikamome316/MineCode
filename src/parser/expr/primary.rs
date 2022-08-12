use nom::branch::alt;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::{IResult, Parser};

use crate::ast::Expr;
use crate::parser::basic::{ident, symbol};

use super::parser::expr;

pub fn _primary(input: &str) -> IResult<&str, Expr> {
    if input.trim().is_empty() {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Eof,
        )));
    }

    let (mut input, mut ret) = alt((
        preceded(symbol('-'), _primary.map(Box::new)).map(Expr::Negative),
        preceded(symbol('~'), _primary.map(Box::new)).map(Expr::BitwiseNot),
        preceded(symbol('!'), _primary.map(Box::new)).map(Expr::LogicalNot),
        // CompileTime
        preceded(symbol('@'), _primary.map(Box::new)).map(Expr::CompileTime),
        // []
        delimited(symbol('['), _primary.map(Box::new), symbol(']')).map(Expr::Pointer),
        // SubExpr
        delimited(symbol('('), _primary.map(Box::new), symbol(')')).map(Expr::SubExpr),
        //
        super::_num,
        super::_for,
        super::_if,
        super::_string,
        super::_ident,
        super::_exprs,
    ))
    .parse(input)?;

    loop {
        let a: Option<Expr>;
        (input, a) = opt(delimited(symbol('['), expr, symbol(']')))(input)?;
        if let Some(index) = a {
            ret = Expr::Subscript(Box::new(ret), Box::new(index));
            continue;
        }

        let a: Option<String>;
        (input, a) = opt(preceded(symbol('.'), ident))(input)?;
        if let Some(name) = a {
            ret = Expr::Attribute(Box::new(ret), name);
            continue;
        }

        let a: Option<Vec<Expr>>;
        (input, a) = opt(delimited(
            symbol('('),
            separated_list0(symbol(','), expr),
            symbol(')'),
        ))(input)?;

        if let Some(args) = a {
            ret = Expr::FuncCall(Box::new(ret), args);
            continue;
        }
        break;
    }
    Ok((input, ret))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_primary_plain() {
        assert_eq!(_primary("a"), Ok(("", Expr::Ident("a".to_string()))));
    }

    #[test]
    fn test_primary_negative() {
        assert_eq!(
            _primary("-a"),
            Ok(("", Expr::Negative(Box::new(Expr::Ident("a".to_string())))))
        );
    }

    #[test]
    fn test_primary_bitwise_not() {
        assert_eq!(
            _primary("~a"),
            Ok(("", Expr::BitwiseNot(Box::new(Expr::Ident("a".to_string())))))
        );
    }

    #[test]
    fn test_primary_logical_not() {
        assert_eq!(
            _primary("!a"),
            Ok(("", Expr::LogicalNot(Box::new(Expr::Ident("a".to_string())))))
        );
    }

    #[test]
    fn test_primary_compile_time() {
        assert_eq!(
            _primary("@a"),
            Ok((
                "",
                Expr::CompileTime(Box::new(Expr::Ident("a".to_string())))
            ))
        );
    }

    #[test]
    fn test_primary_pointer() {
        assert_eq!(
            _primary("[a]"),
            Ok(("", Expr::Pointer(Box::new(Expr::Ident("a".to_string())))))
        );
    }

    #[test]
    fn test_primary_sub_expr() {
        assert_eq!(
            _primary("(a)"),
            Ok(("", Expr::SubExpr(Box::new(Expr::Ident("a".to_string())))))
        );
    }

    #[test]
    fn test_primary_func_call() {
        assert_eq!(
            _primary("a(b, c)"),
            Ok((
                "",
                Expr::FuncCall(
                    Box::new(Expr::Ident("a".to_string())),
                    vec![Expr::Ident("b".to_string()), Expr::Ident("c".to_string())],
                )
            ))
        );
    }

    #[test]
    fn test_primary_subscript() {
        assert_eq!(
            _primary("a[b]"),
            Ok((
                "",
                Expr::Subscript(
                    Box::new(Expr::Ident("a".to_string())),
                    Box::new(Expr::Ident("b".to_string())),
                )
            ))
        );
    }

    #[test]
    fn test_primary_ranged() {
        assert_eq!(
            _primary("a...b"),
            Ok((
                "",
                Expr::Ranged(
                    Box::new(Expr::Ident("a".to_string())),
                    Box::new(Expr::Ident("b".to_string())),
                )
            ))
        );
    }
}