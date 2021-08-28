pub mod config;
pub mod grammar;
pub mod modifier;

use config::GrammarSyntax;
use grammar::Grammar;
use modifier::{CapitalizeModifier, PluralizeModifier};

fn main() {
    let mut grammar = Grammar::from_json("test.json", "agrogro", GrammarSyntax::default())
        .with_modifier("capitalize".to_string(), &CapitalizeModifier {})
        .with_modifier("s".to_string(), &PluralizeModifier {});

    for _ in 0..15 {
        println!("{}", &grammar.flatten());
    }
}
