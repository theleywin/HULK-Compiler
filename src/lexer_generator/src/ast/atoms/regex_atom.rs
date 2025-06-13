use std::fmt::Display;

use crate::ast::atoms::CharSet;

/// Represents a basic atom in a regular expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RegexAtom {
    /// A single character.
    Char(char),
    /// The epsilon (empty string) symbol.
    Epsilon,
}

impl RegexAtom {
    /// Returns a reference to the character if the atom is `Char`, otherwise `None`.
    pub fn as_char(&self) -> Option<&char> {
        if let Self::Char(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

impl From<char> for RegexAtom {
    /// Converts a character into a `RegexAtom::Char`.
    fn from(c: char) -> Self {
        RegexAtom::Char(c)
    }
}

impl Display for RegexAtom {
    /// Formats the atom for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegexAtom::Char(c) => write!(f, "{}", c),
            RegexAtom::Epsilon => write!(f, "\\epsilon"),
        }
    }
}

impl PartialEq<char> for RegexAtom {
    /// Allows comparing a `RegexAtom` to a `char`.
    fn eq(&self, other: &char) -> bool {
        match self {
            RegexAtom::Char(c) => c == other,
            RegexAtom::Epsilon => false,
        }
    }
}

/// Represents a set of characters or a wildcard in a regular expression.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AtomSet {
    /// A character set (e.g., `[a-z]`).
    CharSet(CharSet),
    /// A wildcard that matches any character.
    Wildcard,
}

impl AtomSet {
    /// Returns a reference to the `CharSet` if this is an `AtomSet::CharSet`, otherwise `None`.
    pub fn as_char_set(&self) -> Option<&CharSet> {
        if let Self::CharSet(cs) = self {
            Some(cs)
        } else {
            None
        }
    }
}

impl Display for AtomSet {
    /// Formats the atom set for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AtomSet::CharSet(char_set) => write!(f, "{}", char_set),
            AtomSet::Wildcard => write!(f, "."),
        }
    }
}

impl PartialEq<char> for AtomSet {
    /// Allows comparing an `AtomSet` to a `char`.
    fn eq(&self, other: &char) -> bool {
        match self {
            AtomSet::CharSet(cs) => cs == other,
            AtomSet::Wildcard => true,
        }
    }
}

/// Represents an atomic element in a regex that can be either a `RegexAtom` or an `AtomSet`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchableAtom {
    /// A single atomic character or epsilon.
    Atom(RegexAtom),
    /// A character class or wildcard.
    AtomSet(AtomSet),
}

impl MatchableAtom {
    /// Returns a reference to the `RegexAtom` if this is `MatchableAtom::Atom`, otherwise `None`.
    pub fn as_atom(&self) -> Option<&RegexAtom> {
        if let Self::Atom(atom) = self {
            Some(atom)
        } else {
            None
        }
    }

    /// Returns a reference to the `AtomSet` if this is `MatchableAtom::AtomSet`, otherwise `None`.
    pub fn as_atom_set(&self) -> Option<&AtomSet> {
        if let Self::AtomSet(set) = self {
            Some(set)
        } else {
            None
        }
    }
}

impl Display for MatchableAtom {
    /// Formats the matchable atom for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchableAtom::Atom(atom) => write!(f, "{}", atom),
            MatchableAtom::AtomSet(atom_set) => write!(f, "{}", atom_set),
        }
    }
}

impl PartialEq<char> for MatchableAtom {
    /// Allows comparing a `MatchableAtom` to a `char`.
    fn eq(&self, other: &char) -> bool {
        match self {
            MatchableAtom::Atom(atom) => atom == other,
            MatchableAtom::AtomSet(set) => set == other,
        }
    }
}