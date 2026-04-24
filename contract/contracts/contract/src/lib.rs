#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Env, Symbol, String, Vec, Address
};

#[contract]
pub struct CredentialContract;

#[contracttype]
#[derive(Clone)]
pub struct Credential {
    pub issuer: Address,
    pub data: String,
}

#[contracttype]
pub enum DataKey {
    Credentials(Address), // employee -> Vec<Credential>
}

#[contractimpl]
impl CredentialContract {

    /// Issue a credential to an employee
    pub fn issue_credential(
        env: Env,
        company: Address,
        employee: Address,
        data: String,
    ) {
        // Require company authorization
        company.require_auth();

        let key = DataKey::Credentials(employee.clone());

        let mut credentials: Vec<Credential> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        let credential = Credential {
            issuer: company.clone(),
            data: data.clone(),
        };

        credentials.push_back(credential);
        env.storage().instance().set(&key, &credentials);

        // Emit event
        env.events().publish(
            (symbol_short!("issued"), company, employee),
            data,
        );
    }

    /// Get all credentials of an employee
    pub fn get_credentials(env: Env, employee: Address) -> Vec<Credential> {
        let key = DataKey::Credentials(employee);

        env.storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }

    /// Revoke a credential by index
    pub fn revoke_credential(
        env: Env,
        company: Address,
        employee: Address,
        index: u32,
    ) {
        company.require_auth();

        let key = DataKey::Credentials(employee.clone());

        let mut credentials: Vec<Credential> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        if index >= credentials.len() {
            panic!("Invalid index");
        }

        let removed = credentials.get(index).unwrap();

        // Only issuer can revoke their credential
        if removed.issuer != company {
            panic!("Not authorized");
        }

        credentials.remove(index);
        env.storage().instance().set(&key, &credentials);

        // Emit event
        env.events().publish(
            (symbol_short!("revoked"), company, employee),
            index,
        );
    }
}
