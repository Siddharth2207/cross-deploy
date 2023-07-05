## Cross Deploy CLI
A rust crate to cross deploy Rain Contracts via cli.
Note :- Wont be able to Cross Deploy DISpair contracts 
Install and build crate 
```sh
cargo build
``` 
Checkout the help section to interact with cli by running : 
```sh
cargo run -- --help 
```

To corss deploy contract run 
```sh
cargo run -- deploy-consumer -o mumbai \
        -i 0x7bf60f30a8136160d8fb30d827153f5b4c3da1aa \
        -s 0xb9f09a795f35ce5d26ca653df3aecc1137647883 \
        -d 0xb20dfedc1b12aa6afa308064998a28531a18c714 \
        -I 0x7bf60f30a8136160d8fb30d827153f5b4c3da1aa \
        -S 0xb9f09a795f35ce5d26ca653df3aecc1137647883 \
        -D 0xb20dfedc1b12aa6afa308064998a28531a18c714 \
        -c 0xe82ec43bd3e516878c6df2088e769a02c04984a7
``` 

To get args details run : 
```sh
cargo run -- deploy-consumer --help 
```
to get usage details 
```
Cross Deploy a Rain consumer contract

Usage: cross-deploy deploy-consumer --from-network <ORIGIN_NETWORK> --from-interpreter <FROM_INTERPRETER> --from-store <FROM_STORE> --from-deployer <FROM_DEPLOYER> --to-interpreter <TO_INTERPRETER> --to-store <TO_STORE> --to-deployer <TO_DEPLOYER> --contract-address <CONTRACT_ADDRESS>

Options:
  -o, --from-network <ORIGIN_NETWORK>
          origin Network to deploy contract from [possible values: ethereum, polygon, mumbai]
  -i, --from-interpreter <FROM_INTERPRETER>
          origin network interpreter
  -s, --from-store <FROM_STORE>
          origin network store
  -d, --from-deployer <FROM_DEPLOYER>
          origin network deployer
  -I, --to-interpreter <TO_INTERPRETER>
          origin network interpreter
  -S, --to-store <TO_STORE>
          origin network store
  -D, --to-deployer <TO_DEPLOYER>
          origin network deployer
  -c, --contract-address <CONTRACT_ADDRESS>
          origin network contract address
  -h, --help
  ```