use dumb_contract::contract::SmartContract;
use dumb_contract::platforms::ethereum::EthereumContract;
use web3::transports::Http;
use web3::types::H160;
use web3::Web3;

fn main() {
    let web3 = Web3::new(Http::new("https://rinkeby.infura.io/v3/YOUR_INFURA_PROJECT_ID").unwrap());

    let contract = EthereumContract {
        contract_address: "0x1234567890abcdef1234567890abcdef12345678"
            .parse::<H160>()
            .unwrap(),
        web3,
    };

    match contract.deploy() {
        Ok(_) => println!("Contract deployed successfully!"),
        Err(e) => eprintln!("Failed to deploy contract: {}", e),
    }

    match contract.call("some_function", &[]) {
        Ok(result) => println!("Function call successful: {}", result),
        Err(e) => eprintln!("Function call failed: {}", e),
    }

    match contract.query("some_query") {
        Ok(result) => println!("Query successful: {}", result),
        Err(e) => eprintln!("Query failed: {}", e),
    }
}
