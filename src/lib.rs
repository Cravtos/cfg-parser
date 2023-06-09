use std::collections::HashMap;
use std::collections::HashSet;

type ControlTable = HashMap<char, Vec<Action>>;

pub struct Prods {
    terms: HashSet<char>,
    nonterms: HashSet<char>,
    init: char,
    pub rules: Vec<(char, Vec<char>)>,
    table: ControlTable,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Action {
    Carry(usize),
    Rule(usize),
    Goto(usize),
    Error,
    Exit,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Element {
    Symbol(char),
    State(usize),
}

impl Prods {
    pub fn new(terms: &[char], nonterms: &[char], init: char, table: ControlTable) -> Self {
        Prods {
            terms: HashSet::from_iter(terms.iter().cloned()),
            nonterms: HashSet::from_iter(nonterms.iter().cloned()),
            init,
            rules: vec![],
            table,
        }
    }

    pub fn add_rule(&mut self, from: char, to: &[char]) {
        self.rules.push((from, to.to_vec()))
    }

    /// LR(1)
    pub fn analyze(&self, s: &str) -> Option<Vec<usize>> {
        let mut stack: Vec<Element> = vec![Element::State(0)];
        let mut production: Vec<usize> = Vec::new();

        let mut state: usize;
        let mut idx: usize = 0;
        let mut symbol: char;
        loop {
            symbol = '$';
            if idx < s.len() {
                symbol = s.chars().nth(idx).unwrap();
            }

            match stack.last().expect("there should be element") {
                Element::State(s) => state = *s,
                _ => panic!("no state on top of stack!"),
            }

            let action = self.get_action(state, symbol);
            match action {
                Action::Carry(state) => {
                    stack.push(Element::Symbol(symbol));
                    stack.push(Element::State(state));
                    idx += 1;
                }
                Action::Rule(rule) => {
                    production.push(rule - 1);
                    let (nonterm, terms) = &self.rules[rule - 1];
                    for _ in 0..2 * terms.len() {
                        stack.pop();
                    }
                    match stack.last().expect("there should be element") {
                        Element::State(s) => state = *s,
                        _ => panic!("no state on top of stack!"),
                    }
                    stack.push(Element::Symbol(*nonterm));
                    match self.get_action(state, *nonterm) {
                        Action::Goto(s) => {
                            state = s;
                            stack.push(Element::State(state));
                        }
                        _ => panic!("should be goto action!"),
                    }
                }
                Action::Exit => break,
                Action::Error => return None,
                Action::Goto(_) => panic!("should meet goto"),
            }
        }

        Some(production)
    }
}

// Helper functions
impl Prods {
    fn get_action(&self, state: usize, symbol: char) -> Action {
        self.table.get(&symbol).expect("Symbol is correct")[state].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn control_table() -> ControlTable {
        let mut table: ControlTable = HashMap::new();
        const SYMBOLS: [char; 12] = ['!', '+', '*', '(', ')', 'a', 'b', 'A', 'B', 'T', 'M', '$'];
        for symbol in SYMBOLS {
            table.insert(symbol, vec![Action::Error; 16]);
        }
        table.get_mut(&'!').unwrap()[0] = Action::Carry(2);
        table.get_mut(&'A').unwrap()[0] = Action::Goto(1);

        table.get_mut(&'$').unwrap()[1] = Action::Exit;

        table.get_mut(&'(').unwrap()[2] = Action::Carry(3);
        table.get_mut(&'a').unwrap()[2] = Action::Carry(4);
        table.get_mut(&'b').unwrap()[2] = Action::Carry(5);
        table.get_mut(&'B').unwrap()[2] = Action::Goto(6);
        table.get_mut(&'T').unwrap()[2] = Action::Goto(7);
        table.get_mut(&'M').unwrap()[2] = Action::Goto(8);

        table.get_mut(&'(').unwrap()[3] = Action::Carry(3);
        table.get_mut(&'a').unwrap()[3] = Action::Carry(4);
        table.get_mut(&'b').unwrap()[3] = Action::Carry(5);
        table.get_mut(&'B').unwrap()[3] = Action::Goto(9);
        table.get_mut(&'T').unwrap()[3] = Action::Goto(7);
        table.get_mut(&'M').unwrap()[3] = Action::Goto(8);

        table.get_mut(&'!').unwrap()[4] = Action::Rule(6);
        table.get_mut(&'+').unwrap()[4] = Action::Rule(6);
        table.get_mut(&'*').unwrap()[4] = Action::Rule(6);
        table.get_mut(&')').unwrap()[4] = Action::Rule(6);

        table.get_mut(&'!').unwrap()[5] = Action::Rule(7);
        table.get_mut(&'+').unwrap()[5] = Action::Rule(7);
        table.get_mut(&'*').unwrap()[5] = Action::Rule(7);
        table.get_mut(&')').unwrap()[5] = Action::Rule(7);

        table.get_mut(&'!').unwrap()[6] = Action::Carry(10);

        table.get_mut(&'!').unwrap()[7] = Action::Rule(2);
        table.get_mut(&'+').unwrap()[7] = Action::Carry(11);
        table.get_mut(&')').unwrap()[7] = Action::Rule(2);

        table.get_mut(&'!').unwrap()[8] = Action::Rule(4);
        table.get_mut(&'+').unwrap()[8] = Action::Rule(4);
        table.get_mut(&'*').unwrap()[8] = Action::Carry(12);
        table.get_mut(&')').unwrap()[8] = Action::Rule(4);

        table.get_mut(&')').unwrap()[9] = Action::Carry(13);

        table.get_mut(&'$').unwrap()[10] = Action::Rule(1);

        table.get_mut(&'(').unwrap()[11] = Action::Carry(3);
        table.get_mut(&'a').unwrap()[11] = Action::Carry(4);
        table.get_mut(&'b').unwrap()[11] = Action::Carry(5);
        table.get_mut(&'B').unwrap()[11] = Action::Goto(14);
        table.get_mut(&'T').unwrap()[11] = Action::Goto(7);
        table.get_mut(&'M').unwrap()[11] = Action::Goto(8);

        table.get_mut(&'(').unwrap()[12] = Action::Carry(3);
        table.get_mut(&'a').unwrap()[12] = Action::Carry(4);
        table.get_mut(&'b').unwrap()[12] = Action::Carry(5);
        table.get_mut(&'T').unwrap()[12] = Action::Goto(15);
        table.get_mut(&'M').unwrap()[12] = Action::Goto(8);

        table.get_mut(&'!').unwrap()[13] = Action::Rule(8);
        table.get_mut(&'+').unwrap()[13] = Action::Rule(8);
        table.get_mut(&'*').unwrap()[13] = Action::Rule(8);
        table.get_mut(&')').unwrap()[13] = Action::Rule(8);

        table.get_mut(&'!').unwrap()[14] = Action::Rule(3);
        table.get_mut(&')').unwrap()[14] = Action::Rule(3);

        table.get_mut(&'!').unwrap()[15] = Action::Rule(5);
        table.get_mut(&'+').unwrap()[15] = Action::Rule(5);
        table.get_mut(&')').unwrap()[15] = Action::Rule(5);

        table
    }

    fn init() -> Prods {
        let terms = ['!', '+', '*', '(', ')', 'a', 'b'];
        let nonterms = ['A', 'B', 'T', 'M'];
        let init = 'A';
        let table = control_table();

        let mut prods = Prods::new(&terms, &nonterms, init, table);
        prods.add_rule('A', &['!', 'B', '!']);
        prods.add_rule('B', &['T']);
        prods.add_rule('B', &['T', '+', 'B']);
        prods.add_rule('T', &['M']);
        prods.add_rule('T', &['M', '*', 'T']);
        prods.add_rule('M', &['a']);
        prods.add_rule('M', &['b']);
        prods.add_rule('M', &['(', 'B', ')']);

        // for (i, rule) in prods.rules.iter().enumerate() {
        //     println!(
        //         "{}) {} -> {}",
        //         i + 1,
        //         rule.0,
        //         String::from_iter(rule.1.iter())
        //     );
        // }

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
