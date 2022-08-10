#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Variable{
    pub name: String,
    pub value: Option<String>,
}

impl Variable {
    pub fn new(name: String, value: Option<String>) -> Variable {
        let var = Variable { name: name, value: value};
        return var;
    }

    pub fn get_value(&self) -> Option<String> {
        let s = self.clone();
        let s_value = s.value.unwrap();
        return Some(s_value);
    }



}
