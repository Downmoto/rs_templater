use crate::field::Field;
pub struct Template {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Template {
    pub fn new(name: &str) -> Self {
        Template {
            name: name.to_string(),
            fields: Vec::new(),
        }
    }

    pub fn register_field(mut self, field: Field) -> Self {
        self.fields.push(field);
        self
    }

    pub fn register_fields(mut self, fields: Vec<Field>) -> Self {
        self.fields.extend(fields);
        self
    }
}