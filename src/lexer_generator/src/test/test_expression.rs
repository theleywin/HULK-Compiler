use crate::ast::{expression::Expression, bin_op::BinOp, bin_op::BinaryOperator, un_op::UnOp, un_op::UnaryOperator, atoms::RegexAtom, atoms::MatchableAtom};

#[test]
fn test_display_atom() {
    let atom = MatchableAtom::Atom(RegexAtom::Char('a'));
    let expr = Expression::Atom(atom.clone());
    assert_eq!(expr.to_string(), "a");
}

#[test]
fn test_display_binop_concat() {
    let left = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a'))));
    let right = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b'))));

    let binop = BinOp {
        left,
        right,
        op: BinaryOperator::Concat,
    };

    let expr = Expression::BinOp(binop);
    assert_eq!(expr.to_string(), "(ab)");
}

#[test]
fn test_display_binop_union() {
    let left = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a'))));
    let right = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b'))));

    let binop = BinOp {
        left,
        right,
        op: BinaryOperator::Union,
    };

    let expr = Expression::BinOp(binop);
    assert_eq!(expr.to_string(), "(a|b)");
}

#[test]
fn test_display_unop_kleene() {
    let operand = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a'))));
    let unop = UnOp {
        operand,
        op: UnaryOperator::KleeneStar,
    };

    let expr = Expression::UnOp(unop);
    assert_eq!(expr.to_string(), "a*");
}

#[test]
fn test_display_unop_plus() {
    let operand = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b'))));
    let unop = UnOp {
        operand,
        op: UnaryOperator::Plus,
    };

    let expr = Expression::UnOp(unop);
    assert_eq!(expr.to_string(), "b+");
}

#[test]
fn test_display_unop_question() {
    let operand = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('c'))));
    let unop = UnOp {
        operand,
        op: UnaryOperator::QuestionMark,
    };

    let expr = Expression::UnOp(unop);
    assert_eq!(expr.to_string(), "c?");
}

#[test]
fn test_nested_expression_display() {
    let left = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a'))));
    let right = Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b'))));
    let union = Expression::BinOp(BinOp {
        left,
        right,
        op: BinaryOperator::Union,
    });

    let star = Expression::UnOp(UnOp {
        operand: Box::new(union),
        op: UnaryOperator::KleeneStar,
    });

    assert_eq!(star.to_string(), "(a|b)*");
}

#[test]
fn test_another_nested_expression_display() {
    let a = Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a')));
    let b = Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b')));

    let union = Expression::BinOp(BinOp {
        left: Box::new(a),
        right: Box::new(b),
        op: BinaryOperator::Union,
    });

    let kleene = Expression::UnOp(UnOp {
        operand: Box::new(union),
        op: UnaryOperator::KleeneStar,
    });

    let c = Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('c')));

    let expr = Expression::BinOp(BinOp {
        left: Box::new(kleene),
        right: Box::new(c),
        op: BinaryOperator::Concat,
    });

    assert_eq!(format!("{}", expr), "((a|b)*c)");
}