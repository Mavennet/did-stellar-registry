#![no_std]
use soroban_sdk::{
    contracterror, contractimpl, contracttype, map, vec, Address, Env, Map, String, Symbol, Vec,
};

pub struct DIDStellarRegsitry;

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub enum DataKey {
    Owner(Address),
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct Identity {
    pub owner: Address,
    pub delegates: Map<Address, Delegation>,
    pub attributes: Vec<Attribute>,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct Delegation {
    pub delegate_type: Symbol, // uint validTo:
    pub valid_to: u64,
}

#[contracttype]
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub name: String,
    pub value: String,
    pub valid_to: u64,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotAuthorized = 1,
    DelegateAlreadyExists = 2,
}

#[contractimpl]
impl DIDStellarRegsitry {
    pub fn get_identity(env: Env, user: Address) -> Identity {
        let key = DataKey::Owner(user.clone());

        return env
            .storage()
            .get(&key)
            .unwrap_or(Ok(Identity {
                owner: user.clone(),
                delegates: map![&env],
                attributes: vec![&env],
            }))
            .unwrap();
    }

    pub fn change_identity_owner(env: Env, from: Address, to: Address) -> Address {
        from.require_auth();
        let key = DataKey::Owner(from.clone());

        env.storage().set(
            &key,
            &Identity {
                owner: to.clone(),
                delegates: map![&env],
                attributes: vec![&env],
            },
        );

        return to;
    }

    pub fn add_delegate(
        env: Env,
        user: Address,
        delegate: Address,
        validity: u64,
    ) -> Result<Identity, Error> {
        user.require_auth();
        let mut identity = Self::get_identity(env.clone(), user.clone());

        identity.delegates.set(
            delegate,
            Delegation {
                delegate_type: Symbol::new(&env, "test"),
                valid_to: validity,
            },
        );
        env.storage().set(&DataKey::Owner(user), &identity);
        return Ok(identity);
    }

    pub fn remove_delegate(env: Env, user: Address, delegate: Address) -> Result<Identity, Error> {
        user.require_auth();
        let mut identity = Self::get_identity(env.clone(), user.clone());

        identity.delegates.remove(delegate);

        env.storage().set(&DataKey::Owner(user), &identity);
        return Ok(identity);
    }

    pub fn add_attribute(
        env: Env,
        user: Address,
        name: String,
        value: String,
        validity: u64,
    ) -> Result<Identity, Error> {
        user.require_auth();
        let mut identity = Self::get_identity(env.clone(), user.clone());

        identity.attributes.push_back(Attribute {
            name: name,
            value: value,
            valid_to: validity,
        });

        env.storage().set(&DataKey::Owner(user), &identity);
        return Ok(identity);
    }
}

#[cfg(test)]
mod test;
