/// Describes how `Grammar` interprets the grammar it expends,
/// in terms of operators and syntax:
///
/// - What determines a variable capture
/// - What determines a symbol to be expanded
/// - How are modifiers call upon an expansion
/// - etc.
#[derive(Debug, Clone, Copy)]
pub struct GrammarSyntax {
    /// character starting a non-terminal symbol to be expanded
    pub symbol_start: char,
    /// character ending a non-terminal symbol to be expanded
    pub symbol_end: char,
    /// character starting a capture expression
    pub capture_start: char,
    /// character ending a capture expression
    pub capture_end: char,
    /// character separating the symbol whose expansion is to be captured,
    /// and the name of the new symbol holding the captured value
    pub capture_operator: char,
    /// character separating the symbol, and the modifier(s) to apply to its expansion
    pub modifier_operator: char,
}

impl Default for GrammarSyntax {
    /// The default syntax as documented, and used in the examples
    fn default() -> Self {
        Self {
            symbol_start: '{',
            symbol_end: '}',
            capture_start: '[',
            capture_end: ']',
            capture_operator: '>',
            modifier_operator: ':',
        }
    }
}

impl GrammarSyntax {
    /// Returns whether the given symbol has any modifier applied to it, according to the grammar syntax configuration
    pub fn has_modifier(&self, symbol: &str) -> bool {
        symbol.contains(self.modifier_operator)
    }

    /// Returns whether the given symbol is non-terminal (its value is to be expanded), according to the grammar syntax configuration
    pub fn is_non_terminal(&self, symbol: &str) -> bool {
        symbol.contains(self.symbol_start) && symbol.contains(self.symbol_end)
    }

    /// Returns whether the given symbol is terminal (its value is intrinsic, and will not be expanded), according to the grammar syntax configuration
    pub fn is_terminal(&self, symbol: &str) -> bool {
        !self.is_non_terminal(&symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_modifier_when_true() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.has_modifier("symbol:modifier"))
    }

    #[test]
    fn has_modifier_when_false() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.has_modifier("nomodifier") == false)
    }

    #[test]
    fn is_non_terminal_when_non_terminal() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.is_non_terminal("I {verb} non-terminal"));
    }

    #[test]
    fn is_non_terminal_when_terminal() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.is_non_terminal("I am terminal") == false);
    }

    #[test]
    fn is_terminal_when_non_terminal() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.is_terminal("I {verb} non-terminal") == false);
    }

    #[test]
    fn is_terminal_when_terminal() {
        let syntax = GrammarSyntax::default();
        assert!(syntax.is_terminal("I am terminal"));
    }
}
