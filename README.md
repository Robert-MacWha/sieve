# Sieve
Sieve is a generic crypto fund tracer.  Its goal is to automatically, or with human assistance, find out where funds from a particular address end up.  It should, for example, be able to track eth transfers, ERC20s, bridges, basic mixers, and anything else an aspiring blackhat may use to wash their money.

## Architecture
The core architectural premise of Sieve is to recursively get all outgoing transactions, filter through them for those that are interesting, and repeat.

## Traceable actions
- Eth transfer
  - Self Destruct
- ERC20
  - Mint
  - Transfer
  - TransferFrom
  - "Transfer" event
- NFT
  - "Transfer" event
- Bridging
- Swaps
  - Uniswap
  - Sushi
  - Balancer
  - ...
- Internal token balances (LP)
  - Balancer
  - ...
- Liquidity
  - Deposit
  - Removal
- Vaults
  - Yearn
  - ERC4626
- Wrap / unwrap tokens
- Approvals and Allowances
- Change owner
- Privacy Pools
- CEXs
- Multi-sig wallets

### Challenges
- Track funds in txns involving the relevant party
- Track funds in txns NOT involving the relevant party (IE transferFrom)
- Track funds across ambiguous / multi-party swaps (RFQ)
- Events can lie, function signatures can be different.  Need to make sure value is actually transferred
- 

### Solutions
- Will need to keep track of ALL "interesting" accounts and inspect ALL internal transactions, not just those triggered by or involving (involved within the call trace) the target addresses.
  - IE if the target address BAD approves some address SPENDER for spending tokens, we now need to inspect all of SPENDER's txns as well as all of BAD's
- 