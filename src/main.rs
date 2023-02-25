use parser::Prods;

fn main() {
    let terms = ['!', '+', '*', '(', ')', 'a', 'b'];
    let nonterms = ['А', 'В', 'Т', 'М'];
    let init = 'A';

    let mut prods = Prods::new(&terms, &nonterms, init);

    prods.add_rule('A', &['a']);
    prods.add_rule('А', &['!', 'В', '!']);
    prods.add_rule('В', &['Т']);
    prods.add_rule('В', &['Т', '+', 'В']);
    prods.add_rule('Т', &['М']);
    prods.add_rule('Т', &['М', '*', 'Т']);
    prods.add_rule('М', &['a']);
    prods.add_rule('М', &['b']);
    prods.add_rule('М', &['(', 'В', ')']);

    let input = "!B!";
    let derivation = prods.analyze(input);
    println!("{:?}", derivation);
}