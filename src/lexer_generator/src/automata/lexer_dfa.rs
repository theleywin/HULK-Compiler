use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

use super::lexeme::Lexeme;
use super::lexer_nfa::LexerNFA;
use super::ndt::NDT;
use super::tracker::VisitTracker;
use super::utils::{to_set, to_str};

pub struct LexerDFA<T>
where
    T: Clone + PartialEq,
{
    pub start_state: usize,
    pub accepting_states: HashMap<usize, T>,
    pub transitions: HashMap<(usize, char), usize>,
}

impl<T> LexerDFA<T>
where
    T: Clone + PartialEq + Eq + Hash + Debug,
{
    pub fn new(nfa: &LexerNFA<T>) -> Self {
        LexerDFA::from(nfa)
    }

    pub fn scan<'a>(&self, input: &'a str) -> Result<Vec<Lexeme<'a, T>>, Vec<String>> {
        let mut results = Vec::new();
        let mut issues = Vec::new();
        let source_chars: Vec<char> = input.chars().collect();
        let mut index = 0;
        let mut line_info = (0, 0);
        let total = source_chars.len();

        while index < total {
            let mut state = self.start_state;
            let mut last_match: Option<(usize, &T)> = None;
            let mut line_snapshot = line_info;
            let mut offset = index;

            while offset < total {
                let ch = source_chars[offset];
                if ch == '\n' {
                    line_snapshot.0 += 1;
                    line_snapshot.1 = offset;
                }

                if let Some(&next_state) = self.transitions.get(&(state, ch)) {
                    state = next_state;
                    if let Some(kind) = self.accepting_states.get(&state) {
                        last_match = Some((offset + 1, kind));
                    }
                    offset += 1;
                } else {
                    break;
                }
            }

            if let Some((end, kind)) = last_match {
                let fragment = &input[index..end];
                results.push(Lexeme::with(
                    kind.clone(),
                    fragment,
                    line_info.0,
                    index,
                    end,
                ));
                line_info = line_snapshot;
                index = end;
            } else {
                issues.push(format!(
                    "Lexical Error!: Unexpected character '{}' at line: {}, column: {}",
                    source_chars[index],
                    line_info.0,
                    index - line_info.1
                ));
                line_info = line_snapshot;
                index += 1;
            }
        }

        if issues.is_empty() {
            Ok(results)
        } else {
            Err(issues)
        }
    }
}

impl<T> From<&LexerNFA<T>> for LexerDFA<T>
where
    T: Clone + PartialEq + Eq + Hash,
{
    fn from(nfa: &LexerNFA<T>) -> Self {
        let start_state = 0;
        let mut transitions = HashMap::new();
        let mut tracker = VisitTracker::new();

        let initial = nfa.e_closure(&HashSet::from([nfa.start_state]));
        let id = to_str(&initial);
        tracker.add_unseen(id.clone());

        while let Some(state_id) = tracker.pop_unseen() {
            let state_set = to_set(&state_id);
            for byte in 0..=255u8 {
                let ch = char::from(byte);
                let target_set = nfa.e_closure(&nfa.move_to(&state_set, &ch.into()));

                if target_set.is_empty() {
                    continue;
                }

                let target_id = to_str(&target_set);
                if !tracker.is_seen(&target_id) {
                    tracker.add_unseen(target_id.clone());
                }

                transitions.insert((tracker[&state_id], ch), tracker[&target_id]);
            }
        }

        let accepting_states = tracker
            .iter()
            .filter_map(|sid| {
                let set = to_set(sid);
                set.iter()
                    .filter_map(|q| nfa.accepting_states.get(q).cloned())
                    .min_by_key(|(_, priority)| *priority)
                    .map(|(kind, _)| (tracker[sid], kind))
            })
            .collect();

        LexerDFA {
            start_state,
            accepting_states,
            transitions,
        }
    }
}