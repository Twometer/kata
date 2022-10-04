use crate::instr::Instruction;

pub struct Template {
    instructions: Vec<Instruction>,
}

trait TemplateObject {}

impl Template {
    pub fn compile(input: &str) {}

    pub fn render(&self) -> String {
        String::new()
    }
}
