pub mod template;
pub mod field;

#[cfg(test)]
mod tests {
    use crate::*;
    use field::Field;
    use template::Template;

    #[test]
    fn test_field_creation() {
        let field_name = "test_field";
        let field = Field::new(field_name);

        assert_eq!(field.name, field_name);
        assert!(field.rules.is_empty());
    }

    #[test]
    fn test_field_with_rules() {
        let field_name = "test_field";
        let rule1 = |s: &String| s == &s.to_lowercase();
        let rule2 = |s: &String| s.starts_with("A");

        let field = Field::with_rules(field_name, vec![Box::new(rule1), Box::new(rule2)]);

        assert_eq!(field.name, field_name);
        assert_eq!(field.rules.len(), 2);

        // Test that the rules behave as expected
        let test_str = "abc".to_string();
        assert!((field.rules[0])(&test_str));
        assert!(!(field.rules[1])(&test_str));
    }

    #[test]
    fn test_register_rule() {
        let field_name = "test_field";
        let mut field = Field::new(field_name);

        let rule = |s: &String| s == &s.to_lowercase();
        field = field.register_rule(rule);

        assert_eq!(field.rules.len(), 1);

        let test_str = "abc".to_string();
        assert!((field.rules[0])(&test_str));
    }

    #[test]
    fn test_template_creation() {
        let template_name = "test_template";
        let template = Template::new(template_name);

        assert_eq!(template.name, template_name);
        assert!(template.fields.is_empty());
    }

    #[test]
    fn test_template_register_field() {
        let template_name = "test_template";
        let mut template = Template::new(template_name);

        let field_name = "test_field";
        let field = Field::new(field_name).register_rule(|s| s == &s.to_lowercase());

        template = template.register_field(field);

        assert_eq!(template.fields.len(), 1);
        assert_eq!(template.fields[0].name, field_name);
    }

    #[test]
    fn test_template_with_multiple_fields() {
        let template_name = "test_template";
        let mut template = Template::new(template_name);

        let field1 = Field::new("field1").register_rule(|s| s == &s.to_lowercase());
        let field2 = Field::new("field2").register_rule(|s| s.starts_with("A"));

        template = template.register_field(field1).register_field(field2);

        assert_eq!(template.fields.len(), 2);
        assert_eq!(template.fields[0].name, "field1");
        assert_eq!(template.fields[1].name, "field2");
    }

   
    #[test]
    fn test_field_rules_behavior() {
        let field = Field::with_rules("test_field", vec![
            Box::new(|s: &String| s == &s.to_lowercase()),
            Box::new(|s: &String| s.starts_with("A")),
        ]);

        let lowercase_str = "abc".to_string();
        let uppercase_str = "Abc".to_string();

        assert!((field.rules[0])(&lowercase_str)); // True: "abc" == "abc"
        assert!(!(field.rules[0])(&uppercase_str)); // False: "Abc" != "abc"

        assert!(!(field.rules[1])(&lowercase_str)); // False: "abc" doesn't start with "A"
        assert!((field.rules[1])(&uppercase_str)); // True: "Abc" starts with "A"
    }
}