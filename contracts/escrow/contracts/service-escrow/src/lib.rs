#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum JobStatus {
    Created,
    WorkSubmitted,
    Approved,
    Disputed,
    Completed,
    Refunded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Job {
    pub job_id: u64,
    pub client: Address,
    pub specialist: Address,
    pub description: String,
    pub amount: i128,
    pub status: JobStatus,
}

#[contracttype]
pub enum DataKey {
    Job(u64),
}

#[contract]
pub struct ServiceEscrowContract;

#[contractimpl]
impl ServiceEscrowContract {
    pub fn create_job(
        env: Env,
        job_id: u64,
        client: Address,
        specialist: Address,
        description: String,
        amount: i128,
    ) -> Job {
        client.require_auth();

        if amount <= 0 {
            panic!("amount must be greater than zero");
        }

        let key = DataKey::Job(job_id);

        if env.storage().persistent().has(&key) {
            panic!("job already exists");
        }

        let job = Job {
            job_id,
            client,
            specialist,
            description,
            amount,
            status: JobStatus::Created,
        };

        env.storage().persistent().set(&key, &job);

        job
    }

    pub fn get_job(env: Env, job_id: u64) -> Job {
        let key = DataKey::Job(job_id);

        env.storage()
            .persistent()
            .get(&key)
            .expect("job not found")
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_create_job() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ServiceEscrowContract, ());
        let client_contract = ServiceEscrowContractClient::new(&env, &contract_id);

        let client = Address::generate(&env);
        let specialist = Address::generate(&env);

        let job = client_contract.create_job(
            &1,
            &client,
            &specialist,
            &String::from_str(&env, "Build landing page"),
            &500,
        );

        assert_eq!(job.job_id, 1);
        assert_eq!(job.client, client);
        assert_eq!(job.specialist, specialist);
        assert_eq!(job.amount, 500);
        assert_eq!(job.status, JobStatus::Created);
    }

    #[test]
    fn test_get_job() {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register(ServiceEscrowContract, ());
        let client_contract = ServiceEscrowContractClient::new(&env, &contract_id);

        let client = Address::generate(&env);
        let specialist = Address::generate(&env);

        client_contract.create_job(
            &1,
            &client,
            &specialist,
            &String::from_str(&env, "Build smart contract"),
            &1000,
        );

        let saved_job = client_contract.get_job(&1);

        assert_eq!(saved_job.job_id, 1);
        assert_eq!(saved_job.amount, 1000);
        assert_eq!(saved_job.status, JobStatus::Created);
    }
}
