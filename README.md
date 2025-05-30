# SolVault

A Solana smart contract project built with the Anchor framework that implements a simple vault to securely deposit and withdraw lamports (Solana tokens) using Program Derived Addresses (PDAs).

## Overview

SolVault demonstrates how to create a vault on Solana that allows users to deposit lamports into a program-controlled PDA account and withdraw them securely. It leverages Anchor’s powerful macros and runtime features for simplified Solana development.

## Features

- Deposit lamports into a secure PDA vault
- Withdraw lamports from the vault back to the signer wallet
- Uses Anchor's `#[program]`, `#[derive(Accounts)]`, and error handling macros
- PDA vault derivation based on user wallet address

## Technologies Used

- Rust
- Anchor framework
- Solana blockchain
- TypeScript for testing

## Getting Started

### Prerequisites

- Rust (latest stable)
- Solana CLI
- Anchor CLI
- Node.js and Yarn

### Setup

1. Install dependencies

   ```bash
   yarn install
````
2. Start local Solana test validator

   ```bash
   solana-test-validator
   ```

3. Build and deploy the program

   ```bash
   anchor build
   anchor deploy
   ```

4. Run the tests

   ```bash
   anchor test
   ```

## Project Structure

* `programs/blueshift_anchor_vault` — Rust smart contract code
* `tests` — TypeScript test scripts using Anchor testing framework
* `Anchor.toml` — Anchor configuration file

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

# Contact

For questions or suggestions, feel free to open an issue or contact me.

---

Thank you for checking out SolVault!
