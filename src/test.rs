use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Logger, Address, Env};

extern crate std;
#[cfg(test)]
mod tests {

    use super::*;

    // // Get tests
    #[test]
    fn get_default_identity() {
        let env = Env::default();
        let contract_id = env.register_contract(None, DIDStellarRegsitry);

        let client = DIDStellarRegsitryClient::new(&env, &contract_id);
        let user = Address::random(&env);

        let result = client.get_identity(&user);

        assert_eq!(
            result,
            Identity {
                owner: user,
                delegates: map![&env],
                attributes: vec![&env],
            }
        );
    }

    #[test]
    fn change_owner() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DIDStellarRegsitry);

        let client = DIDStellarRegsitryClient::new(&env, &contract_id);
        let from = Address::random(&env);
        let to = Address::random(&env);

        let logs = env.logger().all();
        std::println!("{}", logs.join("\n"));

        client.change_identity_owner(&from, &to);

        let result = client.get_identity(&from);

        assert_eq!(
            result,
            Identity {
                owner: to,
                delegates: map![&env],
                attributes: vec![&env],
            }
        );
    }

    // Delegates do not affect the identity owner, they are only used for the DIDDocument Delegates fields
    #[test]
    fn add_delegate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DIDStellarRegsitry);

        let client = DIDStellarRegsitryClient::new(&env, &contract_id);
        let user = Address::random(&env);
        let delegate = Address::random(&env);

        client.add_delegate(&user, &delegate, &200);

        let result = client.get_identity(&user);
        assert_eq!(
            result,
            Identity {
                owner: user,
                delegates: map![
                    &env,
                    (
                        delegate,
                        Delegation {
                            delegate_type: Symbol::new(&env, "test"),
                            valid_to: 200,
                        }
                    )
                ],
                attributes: vec![&env],
            }
        );
    }

    #[test]
    fn add_attribute() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, DIDStellarRegsitry);

        let client = DIDStellarRegsitryClient::new(&env, &contract_id);
        let user = Address::random(&env);
        let attribute = Attribute {
            name: String::from_slice(&env, "test_name"),
            value: String::from_slice(&env, "test_value"),
            valid_to: 200,
        };

        client.add_attribute(
            &user,
            &attribute.name,
            &attribute.value,
            &attribute.valid_to,
        );

        let result = client.get_identity(&user);
        assert_eq!(
            result,
            Identity {
                owner: user,
                delegates: map![&env],
                attributes: vec![&env, attribute],
            }
        );
    }
}
