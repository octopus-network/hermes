# Octopus relayer Guide

```
ibc-0       vp          vp        ibc-1
ibc-1-sm    ibc-0-tm    ibc-1-tm  ibc-0-sm
```

## first install dfx


前置条件
install dfx

```bash
dfx start --clean
git clone https://github.com/octopus-network/verification-proxies.git
dfx deploy verification_proxies_backend --mode reinstall

# dfx deploy verification_proxies_backend --mode reinstall output
# verification_proxies_backend: #http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai&id=bkyz2-fmaaa-aaaaa-qaaaq-cai
# 我们这里需要的是 id=bkyz2-fmaaa-aaaaa-qaaaq-cai

$ dfx canister call verification_proxies_backend greet world
```

## install hermes

```bash
git clone https://github.com/octopus-network/hermes.git
git checkout octopus-relayer
cargo build -p ibc-relayer-cli
sudo cp target/debug/hermes $HOME/.hermes/bin
```

## install oyster

```bash
git clone https://github.com/octopus-network/oyster.git
cd oyster
git checkout remove_msg_filter_for_test
go mod tidy
make build # or cd cmd/osyster && go build
sudo cp build/oysterd $HOME/go/bin
````

## install gm

```bash
git clone https://github.com/octopus-network/gm.git
cd gm
git checkout oyster
bin/gm install
cp gm.toml.example $HOME/.gm/gm.toml
cp consumer_section.json $HOME/.gm/
```

### start gm

```bash
gm stop
rm -rf ~/.gm/ibc-*
rm -rf ~/.gm/node-*
gm start
gm hermes keys
```

## 修改hermes配置

modify ~/.hermes/config.toml

这里的canister id 是上面合约部署生成的

add counterparty_id, canister_id for chains:
[[chains]]
id = 'ibc-0'
canister_id = "bkyz2-fmaaa-aaaaa-qaaaq-cai"

counterparty_id = 'ibc-1'

## 创建channel

hermes 配置文件检查

```bash
hermes config validate
```

```bash
hermes create channel --a-chain ibc-0 --b-chain ibc-1 --a-port transfer --b-port transfer --new-client-connection
```

## 启动hermes start

```bash
hermes start
```

### 查询两个链的余额

```bash
oysterd --node tcp://localhost:27030 query bank balances $(oysterd --home ~/.gm/ibc-0 keys --keyring-backend="test" show wallet -a)

oysterd --node tcp://localhost:27040 query bank balances $(oysterd --home ~/.gm/ibc-1 keys --keyring-backend="test" show wallet -a)
```

### ibc跨链转账

```bash
hermes tx ft-transfer --timeout-seconds 1000 --dst-chain ibc-1 --src-chain ibc-0 --src-port transfer --src-channel channel-0 --amount 100000

hermes tx ft-transfer --timeout-seconds 10000 --denom ibc/27A6394C3F9FF9C9DCF5DFFADF9BB5FE9A37C7E92B006199894CF1824DF9AC7C --dst-chain ibc-0 --src-chain ibc-1 --src-port transfer --src-channel channel-0 --amount 100000
```


## for Near chain

to try near-ibc <-> oyster
modify the ibc-1 chain of ~/.hermes/config.toml, add the following fields:

[[chains]]
id = 'ibc-1'
type = 'near'
rpc_addr = 'https://rpc.testnet.near.org'

then modify the code of hermes, then re-compile hermes
find the file crates/relayer/src/chain/near/mod.rs
replace SIGNER_ACCOUNT_TESTNET with your test near account

ys-debug: module_attribute: "ibc_client"
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: InvalidModuleId', crates/relayer/src/chain/near/rpc/tool.rs:413:85
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
