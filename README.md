# 📝 Solana CRUD dApp (Anchor)

A simple decentralized Journal application built on the Solana blockchain using the [Anchor framework](https://www.anchor-lang.com/docs). This dApp demonstrates basic **Create**, **Update**, and **Delete** operations for user-specific journal entries.

---

## 🛠️ Features

- **Create** a new journal entry (unique per user & title)
- **Update** the message of an existing journal entry
- **Delete** a journal entry
- Uses `seeds` and `bump` to derive PDA accounts per user and entry title

---

## 📦 Program Info

- **Program ID**: `coUnmi3oBUtwtd9fjeAvSsJssXh5A5xyPbhpewyzRVF`
- **Language**: Rust (Anchor)
- **Account Model**: Each `JournalEntryState` is stored in a PDA derived from `(title, owner)`.

---

## 📁 Account Structure

### `JournalEntryState`

```text
| Field   | Type     | Description          |
|---------|----------|----------------------|
| owner   | `Pubkey` | Owner of the entry   |
| title   | `String` | Max 50 chars         |
| message | `String` | Max 100 chars        |
```

## 🔧 Instructions

Building:

```bash
anchor build -- -- -Znext-lockfile-bump
```

Run the test validator

```bash
solana-test-validator
```

Deploying

```bash
anchor deploy --provider.cluster localnet
```

Example of expected output:

```text
Deploying cluster: http://127.0.0.1:8899
Upgrade authority: /Users/*******/.config/solana/id.json
Deploying program "crudapp"...
Program path: /crud-app/anchor/target/deploy/crudapp.so...
Program Id: 8QJXi7Jgd512foSuyfV7hReNPTWNqMpxgk1YHCjaJYQb

Deploy success
```

Sync the keys, to prevent mismatches between the Anchor.toml and actual deployed program, and to make sure that 
the Anchor CLI knows which program to interact with when using `anchor test`, `anchor deploy`, or `anchor run`:

```bash
anchor keys sync
```

Example of expected output:
```text
Found incorrect program id declaration in "/crud-app/anchor/programs/crudapp/src/lib.rs"
Updated to 8QJXi7Jgd512foSuyfV7hReNPTWNqMpxgk1YHCjaJYQb

Found incorrect program id declaration in Anchor.toml for the program `crudapp`
Updated to 8QJXi7Jgd512foSuyfV7hReNPTWNqMpxgk1YHCjaJYQb

All program id declarations are synced.
```
