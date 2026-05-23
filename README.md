# SEP-41 Token (SibToken)

A Soroban smart contract implementing the [SEP-41 token standard](https://github.com/stellar/stellar-protocol/blob/master/ecosystem/sep-0041.md) on the Stellar network.

## Token Details

| Property | Value |
|----------|-------|
| Name | SibToken |
| Symbol | SIB |
| Decimals | 18 |

## Project Structure

```text
.
├── contracts
│   └── sep41_token
│       ├── src
│       │   ├── lib.rs          # Module declarations
│       │   ├── our_token.rs    # Contract implementation
│       │   ├── token_trait.rs  # SEP-41 TokenInterface trait
│       │   ├── storage.rs      # Storage key types
│       │   ├── events.rs       # Contract events
│       │   ├── errors.rs       # Error types
│       │   └── test.rs         # Unit tests
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
└── README.md
```

## Contract Interface

### Admin

| Function | Description |
|----------|-------------|
| `initialize(admin, initial_supply)` | Sets the admin address and mints the initial supply to admin. Must be called once before `mint`. |
| `mint(to, amount)` | Mints new tokens to `to`. Requires admin auth. |

### Token

| Function | Description |
|----------|-------------|
| `balance(id)` | Returns the token balance of `id`. |
| `transfer(from, to, amount)` | Transfers `amount` from `from` to `to`. Requires `from` auth. |
| `transfer_from(spender, from, to, amount)` | Transfers `amount` using spender's allowance. Requires `spender` auth. |
| `approve(from, spender, amount, live_until_ledger)` | Sets allowance for `spender` to spend from `from`. Requires `from` auth. |
| `allowance(from, spender)` | Returns the current allowance. |
| `burn(from, amount)` | Burns `amount` from `from`. Requires `from` auth. |
| `burn_from(spender, from, amount)` | Burns `amount` using spender's allowance. Requires `spender` auth. |
| `decimals()` | Returns `18`. |
| `name()` | Returns `"SibToken"`. |
| `symbol()` | Returns `"SIB"`. |

### Errors

| Code | Name | Trigger |
|------|------|---------|
| 1 | `InsufficientFunds` | Balance too low for transfer or burn |
| 2 | `InsufficientAllowance` | Allowance too low for `transfer_from` or `burn_from` |
| 3 | `Unauthorized` | `mint` called before `initialize` |

## Development

### Build

```bash
stellar contract build
```

### Test

```bash
cargo test
# or
make test
```

### Deploy (Testnet)

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/sep41_token.wasm \
  --network testnet \
  --source <identity>
```

After deploying, initialize the contract:

```bash
stellar contract invoke \
  --id <contract-id> \
  --network testnet \
  --source <identity> \
  -- initialize \
  --admin <admin-address> \
  --initial_supply <amount>
```
