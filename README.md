# Employee Credentials on Stellar (Soroban)

## Project Description

This project is a Soroban smart contract built on the Stellar network that enables companies to issue verifiable employee credentials directly on-chain.

The goal is to create a trustless, tamper-proof system where employee records (such as job roles or employment verification) can be issued by companies and independently verified by third parties.

---

## What it does

- Allows a company to issue a credential to an employee wallet address
- Stores credential data on-chain (employee, company, role, timestamp)
- Enables anyone to verify whether a credential exists
- Allows retrieval of credential details for verification purposes

---

## Features

### ✅ On-chain Credential Issuance
Companies can securely issue credentials tied to an employee’s Stellar address.

### 🔐 Authentication
Only the issuing company can create a credential (via `require_auth`).

### 🔍 Public Verification
Anyone can:
- Check if a credential exists
- Retrieve credential details

### ⛓️ Tamper-proof Records
Once issued, credentials are stored immutably on the Stellar blockchain.

### 👤 Decentralized Identity Link
Credentials are linked to employee wallet addresses, eliminating reliance on centralized databases.

---

## Future Improvements

- Support multiple credentials per employee
- Add credential revocation functionality
- Include metadata (e.g., IPFS hash for documents)
- Role-based access control for issuers
- Expiry dates for credentials

---

## Tech Stack

- Rust
- Soroban SDK
- Stellar Blockchain

---

## Use Cases

- Employment verification
- Resume validation
- Professional certifications
- Freelancer work history

---

wallet address GATJXQHOEEBQTXWRXIZE2RHLHAUTM6QTXL2APJM4H753JECC2GMICOGM
contract address CDOKZTNWBQ3UZ6Y6ROX5ANL7GC6GRDXXKFCVS7WZNKRRKAJXZ2YSOITW
https://stellar.expert/explorer/testnet/contract/CDOKZTNWBQ3UZ6Y6ROX5ANL7GC6GRDXXKFCVS7WZNKRRKAJXZ2YSOITW
<img width="1600" height="900" alt="image" src="https://github.com/user-attachments/assets/f2ea4da6-4631-4481-b5b9-6df95027777f" />
