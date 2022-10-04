use crate::template::Template;

mod instr;
mod template;

fn main() {
    let tmp = r"
Search results for '{{ query }}'

{{ foreach result in results }}
    - {{ result.name }}
{{ end }}";

    Template::compile(tmp);
}
