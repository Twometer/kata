use crate::{
    context::{TemplateContext, TemplateValue},
    error::{ParseError, RenderError},
    instr::Instruction,
    scanner::Scanner,
};

#[derive(Debug)]
pub struct Template {
    instructions: Vec<Instruction>,
}

trait TemplateObject {}

impl Template {
    pub fn compile(input: &str) -> Result<Self, ParseError> {
        let mut scanner = Scanner::new(input);
        Ok(Self {
            instructions: Self::parse(&mut scanner, 0)?,
        })
    }

    fn parse(scanner: &mut Scanner, level: usize) -> Result<Vec<Instruction>, ParseError> {
        let mut result = Vec::new();

        while scanner.has_remaining() {
            let plain_text = scanner.consume_until_str("{{");
            result.push(Instruction::Text(plain_text.to_owned()));

            scanner.consume_exact("{{");
            scanner.consume_whitespace();

            if scanner.consume_exact("foreach") {
                scanner.consume_whitespace();
                let var_name = scanner.consume_until_char(' ');
                scanner.consume_whitespace();
                if !scanner.consume_exact("in") {
                    return Err(ParseError::Expected(scanner.index(), "in".to_string()));
                }
                scanner.consume_whitespace();

                let var_value = scanner.consume_until_any_char(&[' ', '}']);
                scanner.consume_whitespace();
                if var_value.is_empty() {
                    return Err(ParseError::Expected(
                        scanner.index(),
                        "collection".to_string(),
                    ));
                }

                if !scanner.consume_exact("}}") {
                    return Err(ParseError::Expected(scanner.index(), "}}".to_string()));
                }

                let sub_instrs = Self::parse(scanner, level + 1)?;
                result.push(Instruction::ForEach(
                    var_name.to_owned(),
                    var_value.to_owned(),
                    sub_instrs,
                ));
            } else if scanner.consume_exact("end") {
                scanner.consume_whitespace();
                return if !scanner.consume_exact("}}") {
                    Err(ParseError::Expected(scanner.index(), "}}".to_string()))
                } else if level == 0 {
                    Err(ParseError::Unexpected(
                        scanner.index(),
                        "{{ end }} instruction".to_string(),
                    ))
                } else {
                    Ok(result)
                };
            } else {
                let param_name = scanner.consume_until_char(' ');
                scanner.consume_whitespace();
                if !scanner.consume_exact("}}") {
                    return Err(ParseError::Expected(scanner.index(), "}}".to_string()));
                }
                let data: Vec<String> = param_name.split(".").map(String::from).collect();
                result.push(Instruction::Parameter(data));
            }
        }

        Ok(result)
    }

    pub fn render(&self, context: &TemplateContext) -> Result<String, RenderError> {
        let mut output = String::new();
        Self::render_to(&self.instructions, &mut output, context)?;
        Ok(output)
    }

    fn render_to(
        instrs: &Vec<Instruction>,
        output: &mut String,
        context: &TemplateContext,
    ) -> Result<(), RenderError> {
        for instr in instrs {
            match instr {
                Instruction::Text(str) => {
                    output.push_str(str);
                }
                Instruction::Parameter(path) => {
                    let mut cur_val = context.get_value(path.get(0).unwrap().as_str());
                    let mut idx = 1;
                    while idx < path.len() {
                        let path_itm = path.get(idx).unwrap();
                        if let Some(val) = cur_val {
                            match val {
                                TemplateValue::String(_) => {
                                    cur_val = Some(val);
                                }
                                TemplateValue::StringArray(_) => {
                                    cur_val = Some(val);
                                }
                                TemplateValue::ObjectArray(_) => {
                                    cur_val = Some(val);
                                }
                                TemplateValue::SubContext(ctx) => {
                                    cur_val = ctx.get_value(&path_itm);
                                }
                                TemplateValue::SubContextRef(ctx) => {
                                    cur_val = ctx.get_value(&path_itm);
                                }
                            }
                        }
                        idx += 1;
                    }

                    if let Some(result) = cur_val {
                        output.push_str(result.as_str())
                    } else {
                        return Err(RenderError::CannotResolve(path.to_owned()));
                    }
                }
                Instruction::ForEach(var_name, collection_key, instrs) => {
                    let collection = context.get_value(collection_key).expect("missing value");

                    match collection {
                        TemplateValue::StringArray(str_arr) => {
                            for obj in str_arr {
                                let mut sub_ctx = context.to_owned();
                                sub_ctx.set_str(var_name.as_str(), obj);

                                Self::render_to(instrs, output, &sub_ctx)?;
                            }
                        }
                        TemplateValue::ObjectArray(obj_arr) => {
                            for obj in obj_arr {
                                let mut obj_ctx = TemplateContext::new();
                                obj.load_into(&mut obj_ctx);

                                let mut sub_ctx = context.to_owned();
                                sub_ctx.set_obj_ref(var_name.as_str(), &obj_ctx);

                                Self::render_to(instrs, output, &sub_ctx)?;
                            }
                        }
                        _ => return Err(RenderError::CannotIterate(collection_key.to_owned())),
                    }
                }
            }
        }

        Ok(())
    }
}
