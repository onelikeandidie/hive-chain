Little project in rust to make a cryptocurrency. It is based on
GeekLaunch's "Build a cryptocurrency in Rust" series, you can watch it
[here](https://youtu.be/vJdT05zl6jk)

This project includes:

- Basic blockchain, blocks and transactions
- Basic Block verification
- Example on how to create genesis blocks
- Example on how to mine blocks

There's still a couple of things missing in this cryptocurrency and its
tests that I would like to add in my offtime, here's a few things:

- Test miners don't take money from any address, instead they take only from
  addresses that transfered any money in the last block in the blockchain
- No way to get the current amount of money on an address (or even a cached
  version)
- Verifications could be a little more extensive
- No Blockchain api to interact with the blockchain from outside
- Blockchain could a use a generic type like `Blockchain<Block>` to support
  other purposes
- Miners could be multithreaded
- Difficulty doesn't update every block
- Block mining times are inconsistent because of the latter
- Fee isn't based on the transactions

I made this for fun, don't go around judging my code :D. If you want to base it
on my code feel free
