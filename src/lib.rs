use std::collections::HashSet;

#[derive(Debug)]
pub struct Prods {
    terms: HashSet<char>,
    nonterms: HashSet<char>,
    init: char,
    pub rules: Vec<(char, Vec<char>)>,
}

#[derive(Clone, Debug)]
struct Derivation {
    pub idxs: Vec<usize>,
    str_pos: usize,
}

#[derive(Clone, Debug)]
struct State {
    sym_pos: usize,
    from_rule: usize,
    drv: Derivation,
}

enum Result {
    Match(Derivation),
    NotMatched,
    NoRules,
}

impl Prods {
    pub fn new(terms: &[char], nonterms: &[char], init: char) -> Self {
        Prods {
            terms: HashSet::from_iter(terms.iter().cloned()),
            nonterms: HashSet::from_iter(nonterms.iter().cloned()),
            init,
            rules: vec![],
        }
    }

    // TODO: check that from and to is from terms or nonterms
    pub fn add_rule(&mut self, from: char, to: &[char]) {
        self.rules.push((from, to.to_vec()))
    }

    fn handle_non_term(&self, rhs: &Vec<char>, s: &str, rule: usize) -> Result {
        let mut prev_states = vec![];

        let mut state = State {
            sym_pos: 0,
            from_rule: 0,
            drv: Derivation { idxs: vec![rule], str_pos: 0 }
        };

        while state.sym_pos < rhs.len() {
            if self.nonterms.contains(&rhs[state.sym_pos]) {
                prev_states.push(state.clone());
            }

            match self._analyze(rhs[state.sym_pos], &s[state.drv.str_pos..], state.from_rule) {
                Result::Match(mut drv) => {
                    state.drv.idxs.append(&mut drv.idxs);
                    state.drv.str_pos += drv.str_pos;
                    state.sym_pos += 1;
                    state.from_rule = 0;
                },
                Result::NotMatched => {
                    // TODO: move to outer fn "restore_prev_state"
                    state = if let Some(prev_state) = prev_states.pop() { 
                        prev_state 
                    } else {
                        return Result::NotMatched;
                    };
                    state.from_rule += 1;
                },
                Result::NoRules => {
                    if prev_states.pop().is_none() { 
                        return Result::NotMatched;
                    };

                    state = if let Some(prev_state) = prev_states.pop() { 
                        prev_state 
                    } else {
                        return Result::NotMatched;
                    };
                    state.from_rule += 1;
                }
            };
        }

        return Result::Match(state.drv);
    }

    fn _analyze(&self, cur: char, s: &str, from_rule: usize) -> Result {
        if self.nonterms.contains(&cur) {
            for (i, (_, rhs)) in self
                .rules
                .iter()
                .enumerate()
                .skip(from_rule)
                .filter(|rule| rule.1 .0 == cur)
            {
                println!("{i}. {cur} -> {rhs:?}");
                match self.handle_non_term(rhs, s, i) {
                    Result::Match(drv) => {
                        return Result::Match(drv);
                    },
                    _ => (),
                }
            }
        }

        if self.terms.contains(&cur) {
            let ch = match s.chars().next() {
                Some(ch) => ch,
                None => return Result::NotMatched,
            };

            if ch == cur {
                return Result::Match(Derivation {
                    idxs: vec![],
                    str_pos: 1,
                });
            }

            return Result::NotMatched;
        }

        Result::NoRules
    }

    // FIXME: возмоно зацикливание для леворекурсивной грамматики
    pub fn analyze(&self, s: &str) -> Option<Vec<usize>> {
        let cur = self.init;
        match self._analyze(cur, s, 0) {
            Result::Match(drv) => Some(drv.idxs),
            _ => None,
        }
    }
}
