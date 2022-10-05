# Kata

Kata (型, Japanese word for _template_) is a string templating library written in Rust. It can render
string templates with {{ curly brace sytnax }} and supports nested objects and lists.

The library aims to be fast by doing very little allocation and copying.

## Example

Template rendering:

```rust
let template = r"
Search results for '{{ query }}'

{{ foreach result in results }}
    - {{ result.name }} at {{ result.url }}
{{ end }}";

let template = Template::compile(tmp)?;

let result_1 = ResultItem::new("Result 1", "https://twometer.cloud");
let result_2 = ResultItem::new("Result 2", "https://google.com");

let mut context = TemplateContext::new();
context.set_str("query", "Test query");
context.set_obj_array("results", &[&result_1, &result_2]);

let rendered = template.render(&context)?;
println!("{}", rendered);
```

Template objects:

```rust
struct ResultItem {
    name: String,
    url: String,
}

impl TemplateObject for ResultItem {
    fn load_into<'a>(&'a self, ctx: &mut TemplateContext<'a>) {
        ctx.set_str("name", &self.name);
        ctx.set_str("url", &self.url);
    }
}
```
