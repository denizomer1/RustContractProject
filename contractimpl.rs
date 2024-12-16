#[contractimpl]
impl TokenContract {
    // Initialize balances for testing purposes
    pub fn initialize(&mut self, env: Env) {
        let alice = Address::from([0; 32]);
        let bob = Address::from([1; 32]);

        self.balances.insert(alice.clone(), 1000);  // Alice starts with 1000 tokens
        self.balances.insert(bob.clone(), 500);    // Bob starts with 500 tokens
    }

    // Freeze account
    pub fn freeze_account_fn(&mut self, env: Env, account: Address) {
        self.freeze_account(&account);
    }

    // Unfreeze account
    pub fn unfreeze_account_fn(&mut self, env: Env, account: Address) {
        self.unfreeze_account(&account);
    }

    // Transfer function
    pub fn transfer_fn(&mut self, env: Env, from: Address, to: Address, amount: u64) -> Result<(), &'static str> {
        self.transfer(&from, &to, amount)
    }

    // Query balance
    pub fn balance(&self, env: Env, account: Address) -> u64 {
        *self.balances.get(&account).unwrap_or(&0)
    }
}
