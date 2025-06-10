use std::fmt::Display;

use crate::ast::atoms::CharSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegexAtom {
    Char(char),
    Epsilon,
}

impl RegexAtom {
    pub fn as_char(&self) -> Option<&char> {
        if let Self::Char(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

impl From<char> for RegexAtom {
    fn from(c: char) -> Self {
        RegexAtom::Char(c)
    }
}

impl Display for RegexAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexAtom::Char(c) => write!(f, "{}", c),
            RegexAtom::Epsilon => write!(f, "\\epsilon"),
        }
    }
}

impl PartialEq<char> for RegexAtom {
    fn eq(&self, other: &char) -> bool {
        match self {
            RegexAtom::Char(c) => c == other,
            RegexAtom::Epsilon => false,
        }
    }
}

pub enum AtomSet {
    CharSet(CharSet),
    Wildcard,
}

impl AtomSet {
    pub fn as_char_set(&self) -> Option<&CharSet> {
        if let Self::CharSet(cs) = self {
            Some(cs)
        } else {
            None
        }
    }
}

impl Display for AtomSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomSet::CharSet(char_set) => write!(f, "{}", char_set),
            AtomSet::Wildcard => write!(f, "."),
        }
    }
}

impl PartialEq<char> for AtomSet {
    fn eq(&self, other: &char) -> bool {
        match self {
            AtomSet::CharSet(cs) => cs == other,
            AtomSet::Wildcard => true,
        }
    }
}

pub enum MatchableAtom {
    Atom(RegexAtom),
    AtomSet(AtomSet),
}

impl MatchableAtom {
    pub fn as_atom(&self) -> Option<&RegexAtom> {
        if let Self::Atom(atom) = self {
            Some(atom)
        } else {
            None
        }
    }

    pub fn as_atom_set(&self) -> Option<&AtomSet> {
        if let Self::AtomSet(set) = self {
            Some(set)
        } else {
            None
        }
    }
}

impl Display for MatchableAtom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchableAtom::Atom(atom) => write!(f, "{}", atom),
            MatchableAtom::AtomSet(atom_set) => write!(f, "{}", atom_set),
        }
    }
}

impl PartialEq<char> for MatchableAtom {
    fn eq(&self, other: &char) -> bool {
        match self {
            MatchableAtom::Atom(atom) => atom == other,
            MatchableAtom::AtomSet(set) => set == other,
        }
    }
}
