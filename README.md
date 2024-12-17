# TestLocker: Solana Token Locking dApp

## Overview

TestLocker is a decentralized application (dApp) built on the Solana blockchain that allows users to lock tokens securely. It supports various locking mechanisms, including standard token locks, vesting locks, and the ability to extend lock durations. This project leverages the Anchor framework for Solana smart contract development, ensuring safety and efficiency.

## Features

- **Token Locking**: Users can lock SPL tokens for a specified duration.
- **Vesting Locks**: Supports vesting schedules, allowing gradual token release over time.
- **Extend Lock Time**: Users can extend the duration of existing locks.
- **Event Emission**: Emits events for lock creation, unlocking, and vesting, enabling easy tracking of actions on-chain.

## Architecture

The project is structured into several modules:

- **Error Handling**: Custom error codes for better debugging and user feedback.
- **Events**: Structs to define events emitted during token locking and unlocking.
- **Instructions**: Contains the core logic for handling various operations, including locking, unlocking, and extending lock times.
- **State Management**: Defines the `LockPda` struct, which holds the state of each lock, including amounts, timings, and user data.
- **Utilities**: Helper functions for token transfers and mint validation.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your machine.
- Solana CLI installed and configured.
- Anchor framework installed.

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/officialnyabuto/testlocker.git
   cd testlocker
   ```

2. Build the project:
   ```bash
   anchor build
   ```

3. Deploy the program to the Solana cluster:
   ```bash
   anchor deploy
   ```

### Running Tests

To ensure the functionality of the dApp, run the provided tests:

```bash
anchor test
```

## Usage

### Locking Tokens

To lock tokens, call the `lock_token` function with the required parameters, including the amount, duration, and additional metadata.

### Unlocking Tokens

Tokens can be unlocked after the specified lock duration using the `unlock_token` function.

### Vesting Tokens

For vesting, use the `lock_vesting` function to set up a vesting schedule, specifying the first release percentage and vesting period.

### Extending Lock Time

Use the `extend_lock_time` function to modify the end time of an existing lock.

## Events

The following events are emitted during operations:

- `CreateLockEvent`: Emitted when a new lock is created.
- `UnlockEvent`: Emitted when tokens are unlocked.
- `LockVestingEvent`: Emitted when a vesting lock is created.
- `UnlockVestingEvent`: Emitted when tokens are unlocked from a vesting lock.
- `ExtendLockTimeEvent`: Emitted when the lock time is extended.

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.

## Resources

For more information, please refer to:
- [Anchor Documentation](https://project-serum.github.io/anchor/)
- [Solana Documentation](https://docs.solana.com/)

## Contact

If you have any questions or suggestions, please open an issue in the GitHub repository.