use chumsky::prelude::*;

#[derive(Debug)]
enum Expr {
    Num(f64),
    Negative(Box<Expr>),
    Invert(Box<Expr>),

    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let number = text::int(10).map(|s: String| Expr::Num(s.parse().unwrap()));

    let oper = |c| just(c).padded();

    let negate_expr = oper('-')
        .repeated()
        .then(number)
        .foldr(move |_, rhs| Expr::Negative(Box::new(rhs)));

    let deref_expr = oper('~')
        .repeated()
        .then(number)
        .foldr(move |_, rhs| Expr::Invert(Box::new(rhs)));

    let unary_expr = deref_expr.or(negate_expr);

    let product_expr = unary_expr
        .then(
            oper('*')
                .to(Expr::Mult as fn(_, _) -> _)
                .or(oper('/').to(Expr::Div as fn(_, _) -> _))
                .then(unary_expr)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

    let sum_expr = product_expr
        .then(
            oper('+')
                .to(Expr::Add as fn(_, _) -> _)
                .or(oper('-').to(Expr::Sub as fn(_, _) -> _))
                .then(product_expr)
                .repeated(),
        )
        .foldl(|lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)));

    sum_expr.padded().then_ignore(end())
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    println!("{:?}", parser().parse(src));
}
