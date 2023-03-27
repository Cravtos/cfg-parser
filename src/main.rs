use parser::Prods;

fn main() {
    let terms = ['a', 'b', '*', '+'];
    let nonterms = ['B', 'T', 'M'];
    let init = 'B';
    let input = "a*b";

    let mut prods = Prods::new(&terms, &nonterms, init);
    prods.add_rule('B', &['B', '+', 'T']);
    prods.add_rule('B', &['T']);
    prods.add_rule('T', &['T', '*', 'M']);
    prods.add_rule('T', &['M']);
    prods.add_rule('M', &['a']);
    prods.add_rule('M', &['b']);

    for (i, rule) in prods.rules.iter().enumerate() {
        println!(
            "{}) {} -> {}",
            i + 1,
            rule.0,
            String::from_iter(rule.1.iter())
        );
    }

    let derivation = prods.analyze(input);
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
