pub enum Instruction {
    Text(String),
    Parameter(Vec<String>),
    ForEach(String, String, Vec<Instruction>),
}
