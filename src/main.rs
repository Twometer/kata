use std::error::Error;

use kata::{Template, TemplateContext, TemplateObject};

struct ResultItem {
    name: String,
    url: String,
}

impl ResultItem {
    fn new(name: &str, url: &str) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}

impl TemplateObject for ResultItem {
    fn load_into<'a>(&'a self, ctx: &mut TemplateContext<'a>) {
        ctx.set_str("name", &self.name);
        ctx.set_str("url", &self.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let tmp = r"
{{ test_obj.name }}
{{ test_obj.url }}
Search results for '{{ query }}'

{{ foreach i in str_arr }}{{ i }},{{ end }}

{{ foreach result in results }}
    - {{ result.name }} at {{ result.url }}
{{ end }}";

    let template = Template::compile(tmp)?;

    let result_1 = ResultItem::new("Result 1", "https://twometer.cloud");
    let result_2 = ResultItem::new("Result 2", "https://google.com");

    let mut context = TemplateContext::new();
    context.set_str("query", "Test query");
    context.set_obj_array("results", &[&result_1, &result_2]);

    context.set_obj("test_obj", &result_1);
    context.set_str_array("str_arr", &["hello", "world"]);

    let rendered = template.render(&context);
    println!("{}", rendered);

    Ok(())
}
