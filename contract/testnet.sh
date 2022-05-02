#!/bin/bash

./build.sh && \
export NEAR_ACCT=advo1.liv1.testnet && \
near delete $NEAR_ACCT liv1.testnet && \
sleep 1 && \
near create-account $NEAR_ACCT --masterAccount liv1.testnet --initialBalance 10 && \
sleep 1 && \
near deploy $NEAR_ACCT --wasmFile ./res/lao.wasm && \
sleep 1 && \
echo "!!! call init contract" && \
near call $NEAR_ACCT init '{}' --accountId liv1.testnet && \
sleep 1 && \
echo "!!! call add_item [new item]" && \
near call $NEAR_ACCT add_item '{"item_id": "item_123", "model":"LastInteraction", "binance": 123, "coinbase":123, "okx":123,"ftx":123,"kraken":123}' --accountId liv1.testnet
echo "!!! call get_item [item]" && \
near call $NEAR_ACCT get_item '{"item_id": "item_123"}' --accountId liv1.testnet
echo "!!! call get_all_items" && \
near call $NEAR_ACCT all_keys '{"paging_start": 0, "paging_size": 0}' --accountId liv1.testnet
