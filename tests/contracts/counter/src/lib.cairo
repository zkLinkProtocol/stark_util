use starknet::ContractAddress;

#[derive(Copy, Drop, Serde, StorageAccess)]
struct ContractInfo {
    block_timestamp: u64,
    contract_address: ContractAddress,
    caller_address: ContractAddress,
}

#[starknet::interface]
trait CounterTrait<T> {
    fn incr(ref self: T);

    fn dec(ref self: T);

    fn register_address(ref self: T, address: ContractAddress);

    fn get_counter(self: @T) -> u64;

    fn is_registered(self: @T, address: ContractAddress) -> bool;

    fn get_counter_status(self: @T) -> (u64, ContractAddress, ContractAddress);

    fn get_contract_info(self: @T) -> ContractInfo;
}

/// @dev Starknet Contract allowing three registered voters to vote on a proposal
#[starknet::contract]
mod Counter {
    use starknet::contract_address::ContractAddressSerde;
    use starknet::ContractAddress;
    use starknet::{get_caller_address, get_contract_address, get_block_timestamp};
    use super::ContractInfo;

    /// @dev Structure that stores vote counts and voter states
    #[storage]
    struct Storage {
        counter: u64,
        registered_address: LegacyMap::<ContractAddress, bool>,
    }

    /// @dev Contract constructor initializing the contract with a list of registered voters and 0 vote count
    #[constructor]
    fn constructor(ref self: ContractState, address: ContractAddress, ) {
        self.registered_address.write(address, true);
        self.counter.write(100);
    }

    #[external(v0)]
    impl CounterImpl of super::CounterTrait<ContractState> {
        fn incr(ref self: ContractState) {
            self.counter.write(self.counter.read() + 1);
        }

        fn dec(ref self: ContractState) {
            self.counter.write(self.counter.read() - 1);
        }

        fn register_address(ref self: ContractState, address: ContractAddress) {
            self.registered_address.write(address, true);
        }

        fn get_counter(self: @ContractState) -> u64 {
            self.counter.read()
        }

        fn is_registered(self: @ContractState, address: ContractAddress) -> bool {
            self.registered_address.read(address)
        }

        fn get_counter_status(self: @ContractState) -> (u64, ContractAddress, ContractAddress) {
            (self.counter.read(), get_contract_address(), get_caller_address())
        }

        fn get_contract_info(self: @ContractState) -> ContractInfo {
            super::ContractInfo {
                block_timestamp: get_block_timestamp(),
                contract_address: get_contract_address(),
                caller_address: get_caller_address(),
            }
        }
    }
}
