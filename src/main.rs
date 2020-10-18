pub mod config;
pub mod random;
pub mod grammar;
pub mod modifier;

use grammar::Grammar;
use config::GrammarSyntax;
use modifier::{
    PluralizeModifier,
    CapitalizeModifier,
};

fn main() {
    let mut grammar = Grammar::from_json(
            "test.json",
            "agrogro",
            GrammarSyntax::default(),
        )
        .with_modifier("capitalize".to_string(), &CapitalizeModifier{})
        .with_modifier("s".to_string(), &PluralizeModifier{});

    for _ in 0..15 {
        println!("{}", &grammar.flatten());
    }
}
