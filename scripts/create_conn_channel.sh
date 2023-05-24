#!/bin/bash -e

# echo "------- create client for rococo-0 ---------"
# hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml create client --host-chain earth-0 --reference-chain rococo-0

# echo "------- create client for earth-0 ---------"
# hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml create client --host-chain rococo-0 --reference-chain earth-0

# echo "------- hermes start ---------"
# nohup hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml start >| ./hermes.log 2>&1 &

echo "------- create connction ---------"
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx conn-init --dst-chain earth-0 --src-chain rococo-0 --dst-client 10-grandpa-0 --src-client 07-tendermint-0
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx conn-try --dst-chain rococo-0 --src-chain earth-0 --dst-client 07-tendermint-0 --src-client 10-grandpa-0 --src-connection connection-0
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx conn-ack --dst-chain earth-0 --src-chain rococo-0 --dst-client 10-grandpa-0 --src-client 07-tendermint-0 --dst-connection connection-0 --src-connection connection-0
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx conn-confirm --dst-chain rococo-0 --src-chain earth-0 --dst-client 07-tendermint-0 --src-client 10-grandpa-0 --dst-connection connection-0 --src-connection connection-0

echo "------- create channel ---------"
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx chan-open-init --dst-chain rococo-0 --src-chain earth-0 --dst-connection connection-0 --dst-port transfer --src-port transfer
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx chan-open-try --dst-chain earth-0 --src-chain rococo-0 --dst-connection connection-0 --dst-port transfer --src-port transfer --src-channel channel-0
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx chan-open-ack --dst-chain rococo-0 --src-chain earth-0 --dst-connection connection-0 --dst-port transfer --src-port transfer --dst-channel channel-0 --src-channel channel-0
sleep 2
hermes --config crates/relayer-cli/tests/fixtures/cos_sub.toml tx chan-open-confirm --dst-chain earth-0 --src-chain rococo-0 --dst-connection connection-0 --dst-port transfer --src-port transfer --dst-channel channel-0 --src-channel channel-0
