use  crate::ast::atoms::{CharSet, RegexAtom, AtomSet, MatchableAtom};

#[test]
fn test_charset_basic_match() {
    let set = CharSet::new(vec![('a', 'c')], false);
    assert!(set == 'a');
    assert!(set == 'b');
    assert!(set == 'c');
    assert!(set != 'd');
}

#[test]
fn test_charset_negation() {
    let set = CharSet::new(vec![('a', 'z')], true);
    assert!(set == '0');
    assert!(set != 'a');
    assert!(set != 'z');
}

#[test]
fn test_charset_mixed_ranges() {
    let set = CharSet::new(vec![('0', '9'), ('a', 'f')], false);
    assert!(set == '0');
    assert!(set == 'c');
    assert!(set != 'g');
}

#[test]
fn test_regex_atom_matching() {
    let a = RegexAtom::Char('x');
    let b = RegexAtom::Char('y');
    let eps = RegexAtom::Epsilon;

    assert!(a == 'x');
    assert!(b != 'x');
    assert!(eps != 'x');
}

#[test]
fn test_atom_set_charset_match() {
    let cs = CharSet::new(vec![('a', 'f')], false);
    let atom_set = AtomSet::CharSet(cs);

    assert!(atom_set == 'a');
    assert!(atom_set == 'd');
    assert!(atom_set != 'z');
}

#[test]
fn test_atom_set_wildcard() {
    let atom_set = AtomSet::Wildcard;

    assert!(atom_set == 'a');
    assert!(atom_set == '9');
    assert!(atom_set == '*');
}

#[test]
fn test_matchable_atom_match_atom() {
    let ma = MatchableAtom::Atom(RegexAtom::Char('k'));
    assert!(ma == 'k');
    assert!(ma != 'x');
}

#[test]
fn test_matchable_atom_match_atomset_charset() {
    let cs = CharSet::new(vec![('1', '3')], false);
    let ma = MatchableAtom::AtomSet(AtomSet::CharSet(cs));

    assert!(ma == '1');
    assert!(ma == '3');
    assert!(ma != '0');
}

#[test]
fn test_matchable_atom_match_atomset_wildcard() {
    let ma = MatchableAtom::AtomSet(AtomSet::Wildcard);

    assert!(ma == 'a');
    assert!(ma == 'Z');
    assert!(ma == '!');
}