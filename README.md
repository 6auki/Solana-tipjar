# TipJar
**TipJar** is a decentralized tipping platform built on the Solana blockchain using the Anchor framework. It allows users to create personalized tip jars, receive SOL with optional messages, and manage tipping goals — all on-chain. This repository contains the core smart contract files (`lib.rs` and `state.rs`), and is the companion to the ["Write Your First Solana Project"](https://medium.com/...) article on Medium.

> ⚠️ **Note:** This is *not* a full Anchor project. This repo includes only `lib.rs` and `state.rs`, which are meant to be integrated into your own Anchor setup.

---

### Example Use Case
  - Alice creates a Tip Jar for her Solana dev blog
  - Bob sends 0.5 SOL with a thank-you message
  - The tip is recorded on-chain; Alice sees the message and goal progress update

---
> If you are new to web3 you can check my [Medium Article]("") where I explain everythin in detail.

---

## Features
  - Create Personalized Tip Jars: Set descriptions, categories, and funding goals
  - Receive Tips in SOL: Accept donations in Solana's native token
  - Public or Anonymous Tips: Choose to reveal or hide the tipper’s identity
  - Attach Messages: Tippers can include personalized messages with their tips
  - Goal Tracking: Set fundraising goals and track progress
  - Efficient Tip History: Circular buffer keeps limited history without excessive storage costs
  - Owner Controls: Pause/resume tips, update details, withdraw funds
  - Security First: Proper authorization checks throughout

---

## Repository Structure
This repository contains only the lib.rs and state.rs code, which are core smart contract codes for the TipJar program. You can integrate into your own Anchor project.

- lib.rs - contains all program logic, instruction handlers, context structs, events, and error definitions.
- state.rs - contains the data structures representing on-chain state.

---

### If You are New to Web3
#### [I Recommend Following this Installation Guide from Anchor](https://www.anchor-lang.com/docs/installation)
- Install Rust
- Install Solana CLI
- Install Anchor

---

##### Create a New Solana Project
```sh
anchor init my-tipjar-project
cd my-tipjar-project
```

### Replace the default lib.rs file:
1. Navigate to **programs/my-tipjar-project/src/**
2. Replace the contents of lib.rs with the provided lib.rs code

### Create a new state.rs file:
1. In the same directory (programs/my-tipjar-project/src/), create a new file named state.rs
2. Copy the contents of the provided state.rs into this file

### Building and Depoying
1. First build
```sh
anchor build
```

2. Update the program ID:
    - After building, a new program ID will be generated
    - Update the declare_id! macro in lib.rs with your program ID

3. Deploy to localnet for testing:
```sh
anchor test
```

4. Deploy to devnet:
```sh
anchor deploy --provider.cluster devnet
```

### Smart Contract Overview
### Program Architecture
The TipJar program is built on Anchor framework and consists of several key components:

1. TipJar Account: A PDA (Program Derived Address) that stores tip jar metadata including owner, description, goal amount, and a history of received tips.
2. Instructions:
    - initialize_tipjar: Creates a new tip jar
    - send_tip: Sends SOL to a tip jar with an optional message
    - withdraw_tip: Allows the owner to withdraw funds
    - toggle_tipjar_status: Pauses or resumes a tip jar
    - update_tipjar: Updates tip jar metadata
    - close_tipjar: Closes a tip jar and returns rent
3. Events: Emitted for frontend tracking and notifications
    - TipSent: When a tip is sent
    - GoalReached: When a tip jar reaches its funding goal
    - TipJarStatusChanged: When a tip jar's status changes
4. Security Considerations:
    - Access control with has_one = owner constraints
    - Input validation with require! statements
    - Withdrawal limits to prevent large, sudden withdrawals


### Circular Buffer for Tips
The program uses a circular buffer to efficiently store tip history without unbounded growth.
This allows the program to maintain a fixed-size history of recent tips while tracking the total count of all tips ever received.
```
if tip_jar.tips_history.len() < TipJar::MAX_TIP_HISTORY_LEN {
    tip_jar.tips_history.push(new_tip.clone());
} else {
    let index = (tip_jar.last_tip_index as usize) % TipJar::MAX_TIP_HISTORY_LEN;
    tip_jar.tips_history[index] = new_tip.clone();
    tip_jar.last_tip_index = ((tip_jar.last_tip_index as usize + 1) % TipJar::MAX_TIP_HISTORY_LEN) as u16;
}
```

# License
This project is licensed under the MIT License - see the LICENSE file for details.
# Contributing
Contributions are welcome! Feel free to open an issue or submit a pull request. :))
