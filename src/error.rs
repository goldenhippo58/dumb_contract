use std::fmt;

#[derive(Debug)]
pub enum ContractError {
    InvalidAddress(String),
    DeploymentFailed(String),
    FunctionCallFailed(String),
    QueryFailed(String),
    NetworkError(String),
}

impl fmt::Display for ContractError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ContractError::InvalidAddress(msg) => write!(f, "Invalid address: {}", msg),
            ContractError::DeploymentFailed(msg) => write!(f, "Deployment failed: {}", msg),
            ContractError::FunctionCallFailed(msg) => write!(f, "Function call failed: {}", msg),
            ContractError::QueryFailed(msg) => write!(f, "Query failed: {}", msg),
            ContractError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for ContractError {}
