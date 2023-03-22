use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
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

#[derive(PartialEq, Eq)]
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
            while let Some(rule) = self.get_conv(&deriv) {
                let (left, right) = &self.rules[rule];
                for _ in 0..right.len() {
                    deriv.pop();
                }
                deriv.push(*left);
                hist.push_front(Record::Rule(rule));
            }

            if idx < s.len() {
                // (2) – carry
                let sym = s.chars().nth(idx).unwrap(); 
                deriv.push(sym);
                idx += 1;

                hist.push_front(Record::Carry);
                continue; // go to (1)
            }

            if deriv.len() == 1 && deriv[0] == self.init {
                // check for (3) condition
                state = State::Ended;
                continue;
            }

            // (4) – reverse state
            state = State::Reverse;

            // (5) – TODO:
        }

        // (3) – normal finish
        unimplemented!()
    }
}

// Helper functions
impl Prods {
    fn get_conv(&self, deriv: &[char]) -> Option<usize> {
        unimplemented!();
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
        println!("{:?}", derivation);
    }
}