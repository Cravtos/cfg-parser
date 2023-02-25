// Недетерминированный синтаксический анализ с возвратами

// Задана следующая порождающая грамматика:
// T = { !, +, *, (, ), a, b }
// N = { А, В, Т, М }, A - стартовый символ

// Продукции:
// 1) А -> !В!
// 2) В -> Т
// 3) В -> Т + В
// 4) Т -> М
// 5) Т -> М * Т
// 6) М -> a
// 7) М -> b
// 8) М -> (В)

// Примеры работы программы:

// 1. !a+b!
// Нисходящий: 1 3 4 6 2 4 7
// Восходящий: 6 4 7 4 2 3 1

// TODO: сделать тесты
// TOOD: сделать вбивание правил вывода
// TODO: разобрать алгоритм восходящего/нисходящего

#[derive(Debug)]
pub struct Prods {
    terms: Vec<char>,
    nonterms: Vec<char>,
    init: char,
    rules: Vec<(char, Vec<char>)>
}

impl Prods {
    pub fn new(terms: &[char], nonterms: &[char], init: char) -> Self {
        Prods { 
            terms: terms.to_vec(),
            nonterms: nonterms.to_vec(),
            init,
            rules: vec![],
        }
    }

    pub fn add_rule(&mut self, from: char, to: &[char]) {
        self.rules.push((from, to.to_vec()))
    }

    pub fn analyze(&self, s: &str) -> &[usize] {
        unimplemented!();
    }
}