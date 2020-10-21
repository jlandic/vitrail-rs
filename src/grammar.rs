use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use crate::config::GrammarSyntax;
use crate::modifier::Modifier;
use crate::random::SeededRng;

/// By default, the grammar will be expanded starting from a symbol named `root`
const DEFAULT_ROOT_KEY: &str = "root";

/// Context-free grammar definition, which can be randomly expanded until all symbols are terminal,
/// based on a given syntax configuration
pub struct Grammar<'a> {
    /// The context-free rules, expressed as a map of symbol => list of possible expansions
    pub symbols: HashMap<String, Vec<String>>,
    /// The syntax to be used to interpret the grammar rules
    pub syntax: GrammarSyntax,
    /// A seeded random number generator instance, to generate reproducible results
    pub rng: SeededRng,
    /// The modifiers featured for the grammar, expressed as a map of modifier name (used as function name in the rules) => the corresponding modifier implementation
    pub modifiers: HashMap<String, &'a dyn Modifier>,
}

impl<'a> Grammar<'a> {
    /// Create a Grammar instance from a grammar described in a JSON file
    ///
    /// ```
    /// use vitrail::{
    ///     config::GrammarSyntax,
    ///     grammar::Grammar,
    /// };
    ///
    /// let grammar = Grammar::from_json(
    ///     "test.json",
    ///     "anyrandomseed",
    ///     GrammarSyntax::default(),
    /// );
    /// ```
    pub fn from_json(file_path: &str, seed: &str, syntax: GrammarSyntax) -> Self {
        let mut file = File::open(file_path)
            .unwrap_or_else(|_| panic!("Could not open grammar file at {}", file_path));
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("Could not read grammar file content.");

        let symbols: HashMap<String, Vec<String>> = serde_json::from_str(&content).unwrap();

        Self {
            symbols,
            syntax,
            rng: SeededRng::new(seed),
            modifiers: HashMap::new(),
        }
    }

    /// Dynamically add a modifier to the Grammar, after its construction.
    ///
    /// The method returns the Grammar instance, so you can build upon it.
    ///
    /// ```
    /// use vitrail::{
    ///     config::GrammarSyntax,
    ///     grammar::Grammar,
    ///     modifier::CapitalizeModifier,
    /// };
    ///
    /// let mut grammar = Grammar::from_json(
    ///     "test.json",
    ///     "anyrandomseed",
    ///     GrammarSyntax::default(),
    /// )
    ///     .with_modifier("capitalize".to_string(), &CapitalizeModifier{});
    /// ```
    pub fn with_modifier(mut self, name: String, modifier: &'a dyn Modifier) -> Self {
        self.modifiers.insert(name, modifier);
        self
    }

    /// Dynamically add a symbol to the Grammar, after its construction.
    ///
    /// The method returns the Grammar instance, so you can build upon it.
    ///
    /// ```
    /// use vitrail::{
    ///     config::GrammarSyntax,
    ///     grammar::Grammar,
    /// };
    ///
    /// let mut grammar = Grammar::from_json(
    ///     "test.json",
    ///     "anyrandomseed",
    ///     GrammarSyntax::default(),
    /// )
    ///     .with_symbol(
    ///         "colour".to_string(),
    ///         vec!["red".to_string(), "blue".to_string(), "yellow".to_string()],
    ///     );
    /// ```
    pub fn with_symbol(mut self, key: String, rules: Vec<String>) -> Self {
        self.symbols.insert(key, rules);
        self
    }

    /// Expand the whole grammar from the default root symbol, until it reaches all terminal
    /// symbols, and return the single expanded string
    pub fn flatten(&mut self) -> String {
        self.flatten_from_root(DEFAULT_ROOT_KEY)
    }

    /// Expand the whole grammar from a given root symbol, until it reaches all terminal
    /// symbols, and return the single expanded string
    pub fn flatten_from_root(&mut self, root: &str) -> String {
        let root_derivation = self.derive_symbol(root);
        self.expand(&root_derivation)
    }

    /// Look for a non-terminal symbol, and return one of its possible expansions in its raw form (without deriving its own value).
    ///
    /// Panics if the symbol cannot be found in the grammar.
    pub fn derive_symbol(&mut self, symbol: &str) -> String {
        match self.symbols.get(symbol) {
            Some(derivations) => self
                .rng
                .random_entry(derivations)
                .unwrap_or_else(|| {
                    panic!(
                        "Unable to expand. Symbol '{}' does not exist in the ruleset.",
                        symbol,
                    )
                })
                .to_string(),
            None => {
                panic!(format!(
                    "Unable to expand. Symbol '{}' does not exist in the ruleset.",
                    symbol,
                ));
            }
        }
    }

    fn expand(&mut self, symbol: &str) -> String {
        if self.syntax.is_terminal(symbol) {
            return symbol.to_string();
        }

        let mut expansion = symbol.to_string();
        while self.syntax.is_non_terminal(&expansion) {
            expansion = self.expand_symbol(&expansion);
        }

        expansion
    }

    fn expand_symbol(&mut self, symbol: &str) -> String {
        let mut symbol_start_idx = 0;
        let mut capture_start_idx = 0;

        for (i, character) in symbol.char_indices() {
            if character == self.syntax.symbol_start {
                symbol_start_idx = i + 1;
            } else if character == self.syntax.capture_start {
                capture_start_idx = i + 1;
            } else if character == self.syntax.symbol_end {
                let key = symbol[symbol_start_idx..i].to_string();
                let expansion = self.expand_non_terminal(&key);

                return format!(
                    "{}{}{}",
                    symbol[0..symbol_start_idx - 1].to_string(),
                    expansion,
                    symbol[i + 1..].to_string(),
                );
            } else if character == self.syntax.capture_end {
                let key = symbol[capture_start_idx..i].to_string();
                self.capture_symbol(&key);

                return symbol.replace(&symbol[capture_start_idx - 1..i + 1], "");
            }
        }

        symbol.to_string()
    }

    fn expand_non_terminal(&mut self, symbol: &str) -> String {
        let operator_idx = symbol.find(self.syntax.modifier_operator);
        let key = match operator_idx {
            Some(idx) => &symbol[0..idx],
            None => symbol,
        };

        let mut derivation = self.derive_symbol(key);
        if self.syntax.has_modifier(symbol) {
            derivation = self.apply_modifier(
                &derivation,
                symbol[operator_idx.unwrap()..]
                    .split(self.syntax.modifier_operator)
                    .collect(),
            )
        }

        derivation
    }

    fn capture_symbol(&mut self, symbol: &str) {
        let capture: Vec<&str> = symbol.split(self.syntax.capture_operator).collect();

        if capture.len() != 2 {
            panic!("Bad capture syntax: '{}'", symbol);
        }

        let new_symbol = capture[1].to_string();
        let extrapolation_key = capture[0];
        let extrapolation = self.derive_symbol(extrapolation_key);

        self.symbols.insert(new_symbol, vec![extrapolation]);
    }

    fn apply_modifier(&self, symbol: &str, modifier_names: Vec<&str>) -> String {
        modifier_names
            .iter()
            .filter_map(|name| self.modifiers.get(*name))
            .fold(symbol.to_string(), |acc, modifier| {
                (*modifier.apply(&acc)).to_string()
            })
    }
}
