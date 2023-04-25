# Goonz-Relay-Balance-Monitor
Monitors the Cryptoon Goonz relayer and notifies slack when the balance of the EOA is too low to pay gas fees for users.

## Installation 

### 0. Install Rust

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 1. Clone this repository 

  `git clone https://github.com/crypdoughdoteth/Goonz-Relay-Balance-Monitor/`

### 2. Create .env file with the following keys:

`API_KEY`, `SLACK_BOT_TOKEN`, `ADDRESS`

API_KEY's corresponding value is your JSON-RPC provider's API key to connect to Ethereum,

SLACK_BOT_TOKEN is for the required access token from Slack,

ADDRESS is the Ethereum address of the account you want to monitor.

### 3. Build it & run
Ensure that you are in the crate's root directory & in your terminal type `cargo run --release`



