use std::collections::HashSet;

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
        let mut hist: Vec<Record> = Vec::new();
        let mut idx = 0;
        let mut state = State::Normal;

        loop {
            match state {
                State::Normal => state = self.handle_normal(&mut deriv, &mut hist, s, &mut idx),
                State::Reverse => state = self.handle_reverse(&mut deriv, &mut hist, s, &mut idx),
                State::Ended => return self.handle_ended(&mut deriv, &mut hist),
            }
        }
    }
}

// Helper functions
impl Prods {
    fn get_conv(&self, deriv: &[char], from: usize) -> Option<usize> {
        for (i, (_, right)) in self.rules.iter().enumerate().skip(from) {
            if deriv.ends_with(right) {
                return Some(i);
            }
        }

        None
    }

    fn handle_normal(
        &self,
        deriv: &mut Vec<char>,
        hist: &mut Vec<Record>,
        s: &str,
        idx: &mut usize,
    ) -> State {
        // (1) – try convolute while possible
        while let Some(rule) = self.get_conv(deriv, 0) {
            let (left, right) = &self.rules[rule];
            for _ in 0..right.len() {
                deriv.pop();
            }
            deriv.push(*left);
            hist.push(Record::Rule(rule));
        }

        // (2) – carry
        if *idx < s.len() {
            let sym = match s.chars().nth(*idx) {
                Some(sym) => sym,
                None => return State::Ended,
            };
            deriv.push(sym);
            *idx += 1;

            hist.push(Record::Carry);
            return State::Normal;
        }

        // (3) – end
        if deriv.len() == 1 && deriv[0] == self.init {
            return State::Ended;
        }

        State::Reverse
    }

    fn handle_reverse(
        &self,
        deriv: &mut Vec<char>,
        hist: &mut Vec<Record>,
        s: &str,
        idx: &mut usize,
    ) -> State {
        // (5а) – pop number and rule and find rule with a higher number matching stack
        match deriv.pop() {
            Some(_) => (),
            None => return State::Ended,
        };
        let record = match hist.pop() {
            Some(record) => record,
            None => return State::Ended,
        };
        let rule = match record {
            Record::Rule(rule) => rule,
            Record::Carry => {
                // (5g) – reverse by terminal
                if *idx == 0 {
                    // Doesn't belong to grammar
                    return State::Ended;
                }
                *idx -= 1;
                return State::Reverse;
            }
        };

        let (_, right) = &self.rules[rule];
        for &c in right {
            deriv.push(c);
        }

        match self.get_conv(deriv, rule + 1) {
            Some(rule) => {
                let (left, right) = &self.rules[rule];
                for _ in 0..right.len() {
                    deriv.pop();
                }
                deriv.push(*left);
                hist.push(Record::Rule(rule));
                State::Normal
            }
            None => {
                // (5b) – everything read and can't find another rule
                if *idx >= s.len() {
                    // already done everything needed
                    return State::Reverse;
                }

                // (5c) – reverse with carry
                let sym = match s.chars().nth(*idx) {
                    Some(sym) => sym,
                    None => return State::Ended,
                };
                deriv.push(sym);
                *idx += 1;

                hist.push(Record::Carry);
                State::Normal
            }
        }
    }

    fn handle_ended(&self, deriv: &mut Vec<char>, hist: &mut Vec<Record>) -> Option<Vec<usize>> {
        if deriv.len() != 1 || deriv[0] != self.init {
            return None;
        }

        let mut result = Vec::new();
        for record in hist.iter() {
            match record {
                Record::Rule(rule) => result.push(*rule),
                Record::Carry => (),
            }
        }
        Some(result)
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
        prods.add_rule('A', &['!', 'B', '!']);
        prods.add_rule('B', &['T']);
        prods.add_rule('B', &['T', '+', 'B']);
        prods.add_rule('T', &['M']);
        prods.add_rule('T', &['M', '*', 'T']);
        prods.add_rule('M', &['a']);
        prods.add_rule('M', &['b']);
        prods.add_rule('M', &['(', 'B', ')']);

        for (i, rule) in prods.rules.iter().enumerate() {
            println!(
                "{}) {} -> {}",
                i + 1,
                rule.0,
                String::from_iter(rule.1.iter())
            );
        }

        prods
    }

    #[test]
    fn test1() {
        let prods = init();
        let input = "!a+b!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "6474231");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test2() {
        let prods = init();
        let input = "!a*b!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "674521");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test3() {
        let prods = init();
        let input = "!(a+b)*(b+a)!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "647423874642384521");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test4() {
        let prods = init();
        let input = "!b*a+a*b!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "76456745231");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test5() {
        let prods = init();
        let input = "!(a+b)*a+b*a!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "64742386457645231");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test6() {
        let prods = init();
        let input = "!(a+b*a)*(b*b+a*(a+b+a))!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "647645238774566474642338452384521");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }

    #[test]
    fn test7() {
        let prods = init();
        let input = "!a+*b!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(_) => {
                panic!("shouldn't belong to grammatic");
            }
            None => (),
        }
    }

    #[test]
    fn test8() {
        let prods = init();
        let input = "a+b*a+b";
        let derivation = prods.analyze(input);

        match derivation {
            Some(_) => {
                panic!("shouldn't belong to grammatic");
            }
            None => (),
        }
    }

    #[test]
    fn test9() {
        let prods = init();
        let input = "a!b";
        let derivation = prods.analyze(input);

        match derivation {
            Some(_) => {
                panic!("shouldn't belong to grammatic");
            }
            None => (),
        }
    }

    #[test]
    fn test10() {
        let prods = init();
        let input = "!a(b+a()!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(_) => {
                panic!("shouldn't belong to grammatic");
            }
            None => (),
        }
    }

    #[test]
    fn test11() {
        let prods = init();
        let input = "!a*(b+a)*b!";
        let derivation = prods.analyze(input);

        match derivation {
            Some(derivation) => {
                let derivation: String = derivation
                    .iter()
                    .map(|rule| (rule + 1).to_string())
                    .collect();
                assert_eq!(derivation, "67464238745521");
            }
            None => {
                panic!("should belong to grammatic");
            }
        }
    }
}
