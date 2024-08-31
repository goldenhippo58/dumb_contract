use crate::error::ContractError;

pub trait SmartContract {
    fn deploy(&self) -> Result<(), ContractError>;
    fn call(&self, function_name: &str, args: &[String]) -> Result<String, ContractError>;
    fn query(&self, query: &str) -> Result<String, ContractError>;
}

// Example of a contract structure
pub struct MyContract {
    pub name: String,
}

impl MyContract {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}
