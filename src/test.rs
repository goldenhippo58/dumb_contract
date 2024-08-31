#[cfg(test)]
mod tests {
    use crate::contract::{MyContract, SmartContract};
    use crate::platforms::binance_smart_chain::BinanceSmartChainContract;
    use crate::platforms::ethereum::EthereumContract;
    use crate::platforms::solana::SolanaContract;
    use solana_client::rpc_client::RpcClient;
    use solana_sdk::signer::keypair::Keypair;

    #[test]
    fn test_ethereum_contract_deploy() {
        let contract = EthereumContract {
            contract_address: "0x1234567890abcdef1234567890abcdef12345678"
                .parse()
                .unwrap(),
            web3: web3::Web3::new(
                web3::transports::Http::new("https://rinkeby.infura.io/v3/YOUR_INFURA_PROJECT_ID")
                    .unwrap(),
            ),
        };
        assert!(contract.deploy().is_ok());
    }

    #[test]
    fn test_solana_contract_deploy() {
        let client = RpcClient::new("https://devnet.solana.com");
        let payer = Keypair::new();
        let contract = SolanaContract::new("program_id", client, payer);

        let result = contract.deploy(); // Add your compiled program binary here
        assert!(result.is_ok());
    }

    #[test]
    fn test_bsc_contract_deploy() {
        let contract = BinanceSmartChainContract {
            contract_address: "0x1234567890abcdef1234567890abcdef12345678"
                .parse()
                .unwrap(),
            web3: web3::Web3::new(
                web3::transports::Http::new("https://bsc-dataseed.binance.org/").unwrap(),
            ),
        };
        assert!(contract.deploy().is_ok());
    }

    #[test]
    fn test_my_contract() {
        let my_contract = MyContract::new("Test Contract");
        assert_eq!(my_contract.name, "Test Contract");
    }

    #[tokio::test]
    async fn test_ethereum_deploy() {
        let contract = EthereumContract::new(
            "0x1234567890abcdef1234567890abcdef12345678",
            "https://rinkeby.infura.io/v3/YOUR_INFURA_PROJECT_ID",
        )
        .await
        .unwrap();
        let receipt = contract.deploy();
        assert!(receipt.is_ok());
    }

    #[tokio::test]
    async fn test_bsc_deploy() {
        let contract = BinanceSmartChainContract::new(
            "0x1234567890abcdef1234567890abcdef12345678",
            "https://bsc-dataseed.binance.org/",
        )
        .await
        .unwrap();
        let receipt = contract.deploy();
        assert!(receipt.is_ok());
    }
}
