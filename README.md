# Glizzy-CLI

Monitors an Ethereum account and notifies slack when the balance of an Ethereum account balance is too low.

## Installation 

### 0. Install Rust

  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### 1. Clone this repository 

  `git clone https://github.com/crypdoughdoteth/Glizzy-CLI/`

### 2. Create .env file with the following keys:

`API_KEY`, `SLACK_BOT_TOKEN`

API_KEY's corresponding value is your JSON-RPC provider's API key to connect to Ethereum,

SLACK_BOT_TOKEN is for the required access token from Slack,

### 3. Build it & run
Ensure that you are in the crate's root directory & in your terminal type `cargo install --path <PATH-TO-FOLDER>`

### 4. Use CLI

`goonz_monitor --address <ADDRESS>`

Optional args include: "threshold" (value) & "chat" (slack channel) 

Optional arg defaults: Threshold = 300, Chat = general

To monitor multiple: _pass `-a` before each address, `-t` before each threshold value_. To set the threshold to be the _same value for each address_, pass *only one* value using the `-t` flag. 

For more help use `goonz_monitor --help`

## Docker 

In the docker-compose file, enter your command with flags/values seperated accordingly:

command: ["glizzy", "-a", "0xAddressHere"]

`docker build -t glizzy .`

`docker-compose up`



