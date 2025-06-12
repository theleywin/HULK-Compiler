use crate::automata::bob_construye_nfa::NfaBuild;
use crate::ast::bin_op::{BinOp, BinaryOperator};
use crate::ast::expression::Expression;
use crate::ast::un_op::{UnOp, UnaryOperator};
use crate::ast::atoms::regex_atom::{RegexAtom, MatchableAtom, AtomSet};
use crate::ast::atoms::charset::CharSet;

#[test]
pub fn match_literal_1() {
    // "abc"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a')))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b')))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('c')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("bc".chars().collect()));
    assert!(!nfa.simulate("abcd".chars().collect()));
}

#[test]
pub fn match_literal_2() {
    // "cab"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('c')))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('a')))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('b')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("cab".chars().collect())); 
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("cb".chars().collect()));
    assert!(!nfa.simulate("c".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
}

#[test]
pub fn match_literal_3() {
    // "xyz"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('x')))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('y')))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('z')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("xyz".chars().collect())); 
    assert!(!nfa.simulate("xy".chars().collect()));
    assert!(!nfa.simulate("yz".chars().collect()));
    assert!(!nfa.simulate("xz".chars().collect()));
}

#[test]
pub fn match_literal_4() {
    // "dog"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('d')))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('o')))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('g')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("dog".chars().collect()));
    assert!(!nfa.simulate("do".chars().collect()));
    assert!(!nfa.simulate("og".chars().collect()));
    assert!(!nfa.simulate("god".chars().collect()));
}

#[test]
pub fn match_literal_5() {
    // "qwe"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('q')))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('w')))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('e')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("qwe".chars().collect()));
    assert!(!nfa.simulate("qw".chars().collect()));
    assert!(!nfa.simulate("qe".chars().collect()));
    assert!(!nfa.simulate("wqe".chars().collect()));
}

#[test]
pub fn match_literal_6() {
    // "hi"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('h')))),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('i')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("hi".chars().collect())); 
    assert!(!nfa.simulate("h".chars().collect()));
    assert!(!nfa.simulate("i".chars().collect()));
    assert!(!nfa.simulate("ih".chars().collect()));
}

#[test]
pub fn match_literal_7() {
    // "go"
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('g')))),
        right: Box::new(Expression::Atom(MatchableAtom::Atom(RegexAtom::Char('o')))),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("go".chars().collect()));
    assert!(!nfa.simulate("g".chars().collect()));
    assert!(!nfa.simulate("o".chars().collect()));
    assert!(!nfa.simulate("og".chars().collect()));
}

#[test]
pub fn match_kleene_star_1() {
    // a*
    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("aa".chars().collect()));
    assert!(nfa.simulate("aaaa".chars().collect()));
    assert!(!nfa.simulate("aaab".chars().collect()));
}

#[test]
pub fn match_kleene_star_2() {
    // (ab)*
    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));     
    assert!(nfa.simulate("ab".chars().collect()));
    assert!(nfa.simulate("abab".chars().collect()));
    assert!(nfa.simulate("ababab".chars().collect()));
    assert!(!nfa.simulate("aba".chars().collect()));
}

#[test]
pub fn match_kleene_star_3() {
    // (abc)*
    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::BinOp(BinOp {
                left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
                right: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
                op: BinaryOperator::Concat,
            })),
            right: Box::new(Expression::Atom(MatchableAtom::Atom('c'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("abcabc".chars().collect()));
    assert!(nfa.simulate("abcabcabc".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
    assert!(!nfa.simulate("abcab".chars().collect()));
}

#[test]
pub fn match_kleene_star_5() {
    // a(bc)*
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
        right: Box::new(Expression::UnOp(UnOp {
            operand: Box::new(Expression::BinOp(BinOp {
                left: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
                right: Box::new(Expression::Atom(MatchableAtom::Atom('c'.into()))),
                op: BinaryOperator::Concat,
            })),
            op: UnaryOperator::KleeneStar,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("abcbc".chars().collect()));
    assert!(nfa.simulate("abcbcbc".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
     assert!(!nfa.simulate("aabc".chars().collect()));
}

#[test]
pub fn match_kleene_star_6() {
    // ((ab)*c)*
    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::UnOp(UnOp {
                operand: Box::new(Expression::BinOp(BinOp {
                    left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
                    right: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
                    op: BinaryOperator::Concat,
                })),
                op: UnaryOperator::KleeneStar,
            })),
            right: Box::new(Expression::Atom(MatchableAtom::Atom('c'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("abababc".chars().collect()));
    assert!(nfa.simulate("abababcc".chars().collect()));
    assert!(nfa.simulate("ccabababcc".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
}

#[test]
pub fn match_kleene_star_7() {
    // ((ab)*c)*d*
    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::UnOp(UnOp {
            operand: Box::new(Expression::BinOp(BinOp {
                left: Box::new(Expression::UnOp(UnOp {
                    operand: Box::new(Expression::BinOp(BinOp {
                        left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
                        right: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
                        op: BinaryOperator::Concat,
                    })),
                    op: UnaryOperator::KleeneStar,
                })),
                right: Box::new(Expression::Atom(MatchableAtom::Atom('c'.into()))),
                op: BinaryOperator::Concat,
            })),
            op: UnaryOperator::KleeneStar,
        })),
        right: Box::new(Expression::UnOp(UnOp {
            operand: Box::new(Expression::Atom(MatchableAtom::Atom('d'.into()))),
            op: UnaryOperator::KleeneStar,
        })),
        op: BinaryOperator::Concat,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("d".chars().collect()));
    assert!(nfa.simulate("dddd".chars().collect()));
    assert!(nfa.simulate("abcabcabcd".chars().collect()));
    assert!(nfa.simulate("abababccdddd".chars().collect()));
    assert!(!nfa.simulate("ab".chars().collect()));
}

#[test]
pub fn match_alpha() {
// [a-zA-Z]*
    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(
            AtomSet::CharSet(CharSet {
                range: vec![('a', 'z'), ('A', 'Z')],
                neg: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("abc".chars().collect()));
    assert!(nfa.simulate("xyz".chars().collect()));
    assert!(nfa.simulate("ABC".chars().collect()));
    assert!(nfa.simulate("XYZ".chars().collect()));
    assert!(nfa.simulate("AbC".chars().collect()));
    assert!(nfa.simulate("aBc".chars().collect()));
    assert!(nfa.simulate("ElBoniatoEsUnaTallaMuyPalPlay".chars().collect()));
    assert!(!nfa.simulate("ABC123".chars().collect()));
    assert!(!nfa.simulate("!A!".chars().collect()));
}

#[test]
pub fn match_non_alpha() {
    // [^a-zA-Z]*

    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(
            AtomSet::CharSet(CharSet {
                range: vec![('a', 'z'), ('A', 'Z')],
                neg: true,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });

    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("123".chars().collect()));
    assert!(!nfa.simulate("xyz".chars().collect()));
    assert!(!nfa.simulate("XYZ".chars().collect()));
    assert!(!nfa.simulate("a".chars().collect()));
    assert!(!nfa.simulate("b".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("ABC".chars().collect()));
    assert!(!nfa.simulate("AbC".chars().collect()));
    assert!(nfa.simulate("@!$%&)()(".chars().collect()));
    assert!(nfa.simulate("1234567890".chars().collect()));
}

#[test]
pub fn match_digit() {
    // [0-9]*

    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(
            AtomSet::CharSet(CharSet {
                range: vec![('0', '9')],
                neg: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });

    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("0".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("123".chars().collect()));
    assert!(nfa.simulate("4567178465762356472357623889".chars().collect()));
    assert!(!nfa.simulate("1235621357e18".chars().collect()));
    assert!(!nfa.simulate("1elefante".chars().collect()));
}

#[test]
pub fn match_dot() {
    // Regex: .
    let regex = Expression::Atom(MatchableAtom::AtomSet(AtomSet::Wildcard));
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("a".chars().collect()));
    assert!(nfa.simulate("b".chars().collect()));
    assert!(nfa.simulate("c".chars().collect()));
    assert!(nfa.simulate("1".chars().collect()));
    assert!(nfa.simulate("!".chars().collect()));
    assert!(nfa.simulate("2".chars().collect()));
    assert!(!nfa.simulate("abc".chars().collect()));
    assert!(!nfa.simulate("".chars().collect()));
}

#[test]
pub fn match_dot_kleene_star() {
    // .*

    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(AtomSet::Wildcard))),
        op: UnaryOperator::KleeneStar,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a2 2e".chars().collect()));
    assert!(nfa.simulate("bebecita ua".chars().collect()));
    assert!(nfa.simulate("sexo anal".chars().collect()));
    assert!(nfa.simulate("3 veces 3".chars().collect()));
    assert!(nfa.simulate(" ".chars().collect()));
    assert!(nfa.simulate("  ".chars().collect()));
    assert!(nfa.simulate("  2  a ".chars().collect()));
}

#[test]
pub fn match_plus_1() {
    // .+

    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(AtomSet::Wildcard))),
        op: UnaryOperator::Plus,
    });
    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(!nfa.simulate("".chars().collect()));
    assert!(nfa.simulate("a2 2e".chars().collect()));
    assert!(nfa.simulate("bebecita ua".chars().collect()));
    assert!(nfa.simulate("sexo anal".chars().collect()));
    assert!(nfa.simulate("3 veces 3".chars().collect()));
    assert!(nfa.simulate(" ".chars().collect()));
    assert!(nfa.simulate("  ".chars().collect()));
    assert!(nfa.simulate("  2  a ".chars().collect()));
}

#[test]
pub fn match_questionmark_concat() {
    // (ab)?

    let regex = Expression::UnOp(UnOp {
        operand: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::Atom(MatchableAtom::Atom('a'.into()))),
            right: Box::new(Expression::Atom(MatchableAtom::Atom('b'.into()))),
            op: BinaryOperator::Concat,
        })),
        op: UnaryOperator::QuestionMark,
    });

    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("".chars().collect()));      
    assert!(nfa.simulate("ab".chars().collect()));    
    assert!(!nfa.simulate("a".chars().collect()));    
    assert!(!nfa.simulate("b".chars().collect()));    
    assert!(!nfa.simulate("abab".chars().collect()));  
    assert!(!nfa.simulate("abc".chars().collect()));
}

#[test]
pub fn match_identifier() {
    // [a-zA-Z][a-zA-Z0-9]*

    let regex = Expression::BinOp(BinOp {
        left: Box::new(Expression::Atom(MatchableAtom::AtomSet(
            AtomSet::CharSet(CharSet {
                range: vec![('a', 'z'), ('A', 'Z')],
                neg: false,
            }),
        ))),
        right: Box::new(Expression::UnOp(UnOp {
            operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(
                AtomSet::CharSet(CharSet {
                    range: vec![('0', '9'), ('a', 'z'), ('A', 'Z')],
                    neg: false,
                }),
            ))),
            op: UnaryOperator::KleeneStar,
        })),
        op: BinaryOperator::Concat,
    });

    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(!nfa.simulate("".chars().collect()));                 
    assert!(nfa.simulate("a".chars().collect()));                 
    assert!(nfa.simulate("A".chars().collect()));                 
    assert!(nfa.simulate("abc".chars().collect()));              
    assert!(nfa.simulate("ABC".chars().collect()));              
    assert!(nfa.simulate("a1b2c3".chars().collect()));           
    assert!(nfa.simulate("identifierSexo69".chars().collect())); 
    assert!(!nfa.simulate("69identifier69".chars().collect())); 
}

#[test]
pub fn match_keyword_while_with_suffix() {
    // "while"[ (\t\r\n)]* (palabra clave seguida de cero o más espacios o paréntesis)

    let keyword = Expression::BinOp(BinOp {
        left: Box::new(Expression::BinOp(BinOp {
            left: Box::new(Expression::BinOp(BinOp {
                left: Box::new(Expression::BinOp(BinOp {
                    left: Box::new(Expression::Atom(MatchableAtom::Atom('w'.into()))),
                    right: Box::new(Expression::Atom(MatchableAtom::Atom('h'.into()))),
                    op: BinaryOperator::Concat,
                })),
                right: Box::new(Expression::Atom(MatchableAtom::Atom('i'.into()))),
                op: BinaryOperator::Concat,
            })),
            right: Box::new(Expression::Atom(MatchableAtom::Atom('l'.into()))),
            op: BinaryOperator::Concat,
        })),
        right: Box::new(Expression::Atom(MatchableAtom::Atom('e'.into()))),
        op: BinaryOperator::Concat,
    });

    let suffix = Expression::UnOp(UnOp {
        operand: Box::new(Expression::Atom(MatchableAtom::AtomSet(
            AtomSet::CharSet(CharSet {
                range: vec![(' ', ' '), ('\t', '\t'), ('\n', '\n'), ('\r', '\r'), ('(', '('), (')', ')')],
                neg: false,
            }),
        ))),
        op: UnaryOperator::KleeneStar,
    });

    let regex = Expression::BinOp(BinOp {
        left: Box::new(keyword),
        right: Box::new(suffix),
        op: BinaryOperator::Concat,
    });

    let mut builder = NfaBuild::new();
    let nfa = builder.build_from_regex(&regex);

    assert!(nfa.simulate("while".chars().collect()));
    assert!(nfa.simulate("while ".chars().collect()));
    assert!(nfa.simulate("while\t".chars().collect()));
    assert!(nfa.simulate("while\n".chars().collect()));
    assert!(nfa.simulate("while\r".chars().collect()));
    assert!(nfa.simulate("while()".chars().collect()));
    assert!(nfa.simulate("while (\n\r\t)".chars().collect()));
}