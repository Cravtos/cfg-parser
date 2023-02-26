use parser::Prods;

fn main() {
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

    for (i, rule) in prods.rules.iter().enumerate() {
        println!("{}) {} -> {}", i, rule.0, String::from_iter(rule.1.iter()));
    }

    let input = "!a+b!";
    let derivation = prods.analyze(input);
    println!("{:?}", derivation);
}
