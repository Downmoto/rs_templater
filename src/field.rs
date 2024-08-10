pub type Rule = Box<dyn Fn(&String) -> bool>;
pub struct Field {
    pub name: String,
    pub rules: Vec<Rule>,
}

impl Field {
    pub fn new(name: &str) -> Self {
        Field {
            name: name.to_string(),
            rules: Vec::new(),
        }
    }

    pub fn with_rules(name: &str, rules: Vec<Rule>) -> Self {
        Field {
            name: name.to_string(),
            rules,
        }
    }

    pub fn register_rule<F>(mut self, rule: F) -> Self
    where
        F: Fn(&String) -> bool + 'static,
    {
        self.rules.push(Box::new(rule));
        self
    }
}
