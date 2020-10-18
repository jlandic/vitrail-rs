#[derive(Debug)]
pub struct GrammarSyntax {
    pub symbol_start: char,
    pub symbol_end: char,
    pub capture_start: char,
    pub capture_end: char,
    pub capture_operator: char,
    pub modifier_operator: char,
}

impl Default for GrammarSyntax {
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
    pub fn has_modifier(&self, symbol: &str) -> bool {
        symbol.contains(self.modifier_operator)
    }

    pub fn is_non_terminal(&self, symbol: &str) -> bool {
        symbol.contains(self.symbol_start) && symbol.contains(self.symbol_end)
    }

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
