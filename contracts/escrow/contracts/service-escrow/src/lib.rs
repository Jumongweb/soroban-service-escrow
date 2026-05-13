#![no_std]

use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct ServiceEscrowContract;

#[contractimpl]
impl ServiceEscrowContract {
    pub fn hello(_env: Env) -> Symbol {
        symbol_short!("escrow")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::Env;

    #[test]
    fn test_hello() {
        let env = Env::default();
        let contract_id = env.register(ServiceEscrowContract, ());
        let client = ServiceEscrowContractClient::new(&env, &contract_id);

        assert_eq!(client.hello(), symbol_short!("escrow"));
    }
}
