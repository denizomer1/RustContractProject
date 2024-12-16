# RustContractProject
# Token Contract with Freeze and Unfreeze Functionality

## Overview
This contract is a simple token contract built on the Stellar blockchain with added functionality to freeze and unfreeze accounts. A frozen account cannot send or receive tokens until it is unfrozen.

## Features
- **Freeze Account**: Temporarily prevents transfers for a given account.
- **Unfreeze Account**: Allows an account to be unfrozen and resume normal token operations.
- **Transfer Blocked for Frozen Accounts**: If a sender or recipient's account is frozen, token transfers are rejected.

## Deployment
The contract has been deployed to the Stellar testnet. The contract address is:

`[Insert contract address here]`

## How to Use
- Use the `freeze_account` function to freeze an account.
- Use the `unfreeze_account` function to unfreeze an account.
- Use the `transfer` function to send tokens between accounts.

## Testing
1. Deploy the contract to the Stellar testnet.
2. Test freezing and unfreezing accounts.
3. Attempt transfers from frozen accounts to verify that they are blocked.

## Contract Address
`[Insert contract address here]`
