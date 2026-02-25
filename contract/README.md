# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

## Payment Flow Events

The contract emits the following events for fund flow tracking:

- `emit_distribution(env, group_id, sender, token, total_amount, member_count)`: Emitted when funds are split and sent to group members.
- `emit_contribution(env, group_id, contributor, token, amount)`: Emitted when someone contributes to a fundraiser.

These events are essential for the frontend transaction history page and analytics dashboard to display real-time payment activity.

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.