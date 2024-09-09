#[derive(Debug)]
pub struct Symbol {
    pub name: String,
}

impl Symbol {
    pub fn new(name: String) -> Symbol {
        Symbol { name }
    }

    pub fn list() -> Vec<Symbol> {
        vec![
            Symbol::new("AAPL".to_string()),
            Symbol::new("GOOGL".to_string()),
            Symbol::new("MSFT".to_string()),
        ]
    }
}
