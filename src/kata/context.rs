use std::collections::HashMap;

pub trait TemplateObject {
    fn load_into<'a>(&'a self, ctx: &mut TemplateContext<'a>);
}

#[derive(Clone)]
pub(crate) enum TemplateValue<'a> {
    String(&'a str),
    StringArray(Vec<&'a str>),
    ObjectArray(Vec<&'a dyn TemplateObject>),
    SubContext(TemplateContext<'a>),
    SubContextRef(&'a TemplateContext<'a>),
}

impl<'a> TemplateValue<'a> {
    pub fn as_str(&self) -> &'a str {
        match self {
            TemplateValue::String(val) => val,
            TemplateValue::StringArray(_) => "[string_arr]",
            TemplateValue::ObjectArray(_) => "[object_arr]",
            TemplateValue::SubContext(_) => "[object]",
            TemplateValue::SubContextRef(_) => "[object_ref]",
        }
    }
}

#[derive(Clone)]
pub struct TemplateContext<'a> {
    values: HashMap<String, TemplateValue<'a>>,
}

impl<'a> TemplateContext<'a> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set_str(&mut self, key: &str, val: &'a str) {
        self.values
            .insert(key.to_owned(), TemplateValue::String(val));
    }

    pub fn set_obj(&mut self, key: &str, obj: &'a impl TemplateObject) {
        let mut sub_context = TemplateContext::new();
        obj.load_into(&mut sub_context);

        self.values
            .insert(key.to_owned(), TemplateValue::SubContext(sub_context));
    }

    pub fn set_str_array(&mut self, key: &str, val: &[&'a str]) {
        self.values
            .insert(key.to_owned(), TemplateValue::StringArray(val.to_vec()));
    }

    pub fn set_obj_array(&mut self, key: &str, val: &[&'a impl TemplateObject]) {
        self.values.insert(
            key.to_owned(),
            TemplateValue::ObjectArray(val.iter().map(|x| *x as &'a dyn TemplateObject).collect()),
        );
    }

    pub(crate) fn set_obj_ref(&mut self, key: &str, val: &'a TemplateContext<'a>) {
        self.values
            .insert(key.to_owned(), TemplateValue::SubContextRef(val));
    }

    pub(crate) fn get_value(&self, key: &str) -> Option<&TemplateValue<'a>> {
        self.values.get(key)
    }
}
