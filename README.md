# 🚀 Token Airdrop Tool (Soroban Smart Contract)

## 📌 Project Description

The **Token Airdrop Tool** is a Soroban smart contract built on the Stellar network that enables fast, secure, and efficient distribution of tokens to multiple recipients in a single transaction.

This project is designed to simplify large-scale token distribution for Web3 projects, communities, and developers by reducing manual effort and transaction overhead.

---

## ⚡ What it does

* Distributes tokens to multiple wallet addresses in one transaction
* Eliminates the need for repetitive manual transfers
* Uses Soroban’s smart contract capabilities for secure execution
* Ensures only authorized users can initiate airdrops

---

## ✨ Features

* 📤 **Batch Airdrop**
  Send tokens to multiple recipients in a single function call

* 🔐 **Secure Authorization**
  Uses `require_auth()` to verify sender identity

* ⚡ **Efficient Transactions**
  Reduces gas cost and execution time compared to multiple transfers

* 🔁 **Single Transfer Support**
  Allows direct transfer to one address

* 🧩 **Modular Design**
  Easy to extend with additional features

---

## 🛠️ Tech Stack

* **Language:** Rust
* **Framework:** Soroban SDK
* **Blockchain:** Stellar Network

---

## 🧠 How it Works

1. The sender provides:

   * Token contract address
   * Sender wallet address
   * List of recipients and token amounts

2. The smart contract:

   * Verifies sender authorization
   * Iterates through the recipient list
   * Transfers tokens to each address

---

## 📄 Contract Functions

### 🔹 `airdrop`

Distributes tokens to multiple recipients

**Parameters:**

* `token` → Token contract address
* `from` → Sender address
* `recipients` → List of recipient addresses with amounts

---

### 🔹 `single_transfer`

Transfers tokens to a single recipient

**Parameters:**

* `token` → Token contract address
* `from` → Sender address
* `to` → Recipient address
* `amount` → Token amount

---

## ⚙️ Installation & Build

```bash
# Clone the repository
git clone https://github.com/your-username/airdrop-contract

# Navigate into the project
cd airdrop-contract

# Build the contract
soroban contract build
```

---

## 🚀 Deployment

Deploy the contract on Stellar testnet:

```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/airdrop_contract.wasm \
  --source YOUR_ACCOUNT \
  --network testnet
```

---

## 🌐 Deployed Smart Contract Link

👉 https://stellar.expert/explorer/testnet/contract/CBQTD7EPURN2M5XQ2AFDZ2AT7MFGVMK5GAAGXPNTZOKKPVFBLPCGIVUO

<img width="1884" height="870" alt="Screenshot 2026-03-19 143429" src="https://github.com/user-attachments/assets/98afd4cb-998d-46e3-99b8-2de48f822dc8" />

---

## 🧪 Future Improvements

* 📦 Merkle Tree Airdrop (gas-efficient claims)
* ⏳ Time-locked token distribution
* 📊 Event logging for tracking airdrops
* 🎯 Admin-controlled campaigns
* 🌐 Frontend dashboard integration

---

## 🤝 Contributing

Contributions are welcome!
Feel free to fork the repository and submit a pull request.

---

Author- ARIJIT DEBNATH

  
## 📜 License

This project is licensed under the **MIT License**.

---

## ⭐ Acknowledgment

Built using the powerful **Soroban SDK** on the Stellar network to enable scalable and efficient smart contract development.
