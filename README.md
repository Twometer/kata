# Kata

Kata (型, Japanese for mould or template) is a string templating library written in Rust. It can render
mustache-like string templates and supports nested objects and lists.

## Example

```
Search results for '{{ query }}'

{{ foreach result in results }}
    - {{ result.name }}
{{ end }}

```
