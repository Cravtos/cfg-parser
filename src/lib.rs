use std::collections::{HashSet, VecDeque};

pub struct Prods {
    terms: HashSet<char>,
    nonterms: HashSet<char>,
    init: char,
    pub rules: Vec<(char, Vec<char>)>,
}

#[derive(PartialEq, Eq)]
enum State {
    Normal,
    Reverse,
    Ended,
}

#[derive(PartialEq, Eq, Debug)]
enum Record {
    Carry,
    Rule(usize),
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

    pub fn add_rule(&mut self, from: char, to: &[char]) {
        self.rules.push((from, to.to_vec()))
    }

    /// Восходящий анализ.
    /// Грамматика должны быть без циклов и неукорачивающейся.
    pub fn analyze(&self, s: &str) -> Option<Vec<usize>> {
        let mut deriv: Vec<char> = Vec::new();
        let mut hist: VecDeque<Record> = VecDeque::new();
        let mut idx = 0;
        let mut state = State::Normal;

        while state != State::Ended {
            // (1) – try convolute while possible
            while let Some(rule) = self.get_conv(&deriv, 0) {
                let (left, right) = &self.rules[rule];
                for _ in 0..right.len() {
                    deriv.pop();
                }
                deriv.push(*left);
                hist.push_front(Record::Rule(rule));
                // println!("(1) Got rule {rule} ({left}->{right:?}); Deriv: {deriv:?}, hist head: {:?}", hist[0]);
            }

            if idx < s.len() {
                // (2) – carry
                let sym = s.chars().nth(idx).expect("There is symbols");
                deriv.push(sym);
                idx += 1;

                hist.push_front(Record::Carry);
                // println!("(2) Symbol: {sym}, Deriv: {deriv:?}, idx={idx}; hist head: {:?}", hist[0]);
                continue; // go to (1)
            }

            if deriv.len() == 1 && deriv[0] == self.init {
                // check for (3) condition
                state = State::Ended;
                continue;
            }

            // (4) – reverse state
            state = State::Reverse;

            // (5) -- State::Reverse
            while state == State::Reverse {
                // (5а) – pop number and rule and find rule with a higher number matching stack
                let non_term = deriv.pop().expect("There is non terminal in derviation");
                let rule = match hist.pop_front().expect("There is records in history") {
                    Record::Rule(rule) => rule,
                    Record::Carry => {
                        // (5g) – reverse by terminal
                        // println!("(5g)");
                        if idx == 0 {
                            // Doesn't belong to grammar
                            return None;
                        }
                        idx -= 1;
                        continue; // to reverse
                    }
                };
                let (left, right) = &self.rules[rule];
                for &c in right {
                    deriv.push(c);
                }

                #[cfg(debug_assertions)]
                assert_eq!(non_term, *left); // TODO: for debug purpose

                match self.get_conv(&deriv, rule + 1) {
                    Some(rule) => {
                        let (left, right) = &self.rules[rule];
                        for _ in 0..right.len() {
                            deriv.pop();
                        }
                        deriv.push(*left);
                        hist.push_front(Record::Rule(rule));
                        state = State::Normal;
                        // println!("(5a)");
                        continue;
                    }
                    None => {
                        // (5b) – everything read and can't find another rule
                        if idx >= s.len() {
                            // already done everything needed
                            // NOTE: staying in reverse state
                            // println!("(5b)");
                            continue;
                        }

                        // (5c) – reverse with carry
                        let sym = s.chars().nth(idx).expect("There is symbols");
                        deriv.push(sym);
                        idx += 1;

                        hist.push_front(Record::Carry);
                        state = State::Normal;
                        // println!("(5c)");
                        continue;
                    }
                }
            }
        }

        // (3) – normal finish
        // println!("(3)");
        let mut result = Vec::new();
        for record in hist {
            match record {
                Record::Rule(rule) => result.push(rule),
                Record::Carry => (),
            }
        }
        Some(result)
    }
}

// Helper functions
impl Prods {
    fn get_conv(&self, deriv: &[char], from: usize) -> Option<usize> {
        for (i, (_, right)) in self.rules.iter().enumerate().skip(from) {
            if deriv.ends_with(&right) {
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> Prods {
        let terms = ['!', '+', '*', '(', ')', 'a', 'b'];
        let nonterms = ['A', 'B', 'T', 'M'];
        let init = 'A';

        let mut prods = Prods::new(&terms, &nonterms, init);
        prods.add_rule('A', &['a']);
        prods.add_rule('A', &['!', 'B', '!']);
        prods.add_rule('B', &['T']);
        prods.add_rule('B', &['T', '+', 'B']);
        prods.add_rule('T', &['M']);
        prods.add_rule('T', &['M', '*', 'T']);
        prods.add_rule('M', &['a']);
        prods.add_rule('M', &['b']);
        prods.add_rule('M', &['(', 'B', ')']);

        prods
    }

    #[test]
    fn test1() {
        let prods = init();
        let input = "!a*b!";
        let derivation = prods.analyze(input);
        for (i, rule) in prods.rules.iter().enumerate() {
            println!(
                "{}) {} -> {}",
                i + 1,
                rule.0,
                String::from_iter(rule.1.iter())
            );
        }

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                println!("Derivation: {derivation}");
            }
            None => {
                println!("Doesn't belong to grammatic");
            }
        }
    }
}
