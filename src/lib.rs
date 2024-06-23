#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait WorldContract {
    #[init]
    fn init(&self) {
        // Executed when you first deploy your smart contract
    }

    #[upgrade]
    fn upgrade(&self) {
        // Executed on every following upgrade
    }

    // --

    #[endpoint(sayHello)]
    fn say_hello(&self) -> ManagedBuffer {
        let greeting = ManagedBuffer::from("Hello, MultiversX!");

        greeting
    }

    // --

    #[endpoint(calculate)]
    fn add(&self, first: u32, second: u32) -> u32 {
        first + second
    }

    // --

    #[storage_mapper("greeting")]
    fn greeting(&self) -> SingleValueMapper<ManagedBuffer>;

    #[endpoint(sayHelloToNextCaller)]
    fn say_hello_to_next_caller(&self, greeting: ManagedBuffer) -> ManagedBuffer {
        let current_greeting = if self.greeting().is_empty() {
            ManagedBuffer::from("Hello, MultiversX!")
        } else {
            self.greeting().get()
        };

        self.greeting().set(greeting);

        current_greeting
    }

    // --

    #[event("greeted")]
    fn greeted_event(&self, #[indexed] caller: ManagedAddress, #[indexed] greeting: ManagedBuffer);

    #[endpoint(sayHelloToOffChain)]
    fn say_hello_to_off_chain(&self, greeting: ManagedBuffer) {
        let caller = self.blockchain().get_caller();

        self.greeted_event(caller, greeting);
    }

    // --

    #[payable("*")]
    #[endpoint(sayHelloOnlyWhenPaid)]
    fn say_hello_only_when_paid(&self, greeting: ManagedBuffer) -> ManagedBuffer {
        let value = self.call_value().egld_value().clone_value();

        require!(value > 0, "You must pay to get a greeting!");

        greeting
    }

    // --

    #[only_owner]
    #[endpoint(payUser)]
    fn pay_user(&self, user_address: ManagedAddress, amount: BigUint) {
        let contract_address = self.blockchain().get_sc_address();
        let balance = self.blockchain().get_balance(&contract_address);

        require!(balance >= amount, "Insufficient funds!");

        self.tx().to(user_address).egld(amount).transfer();
    }
}
