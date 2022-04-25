#!/bin/bash

./build.sh && \
export NEAR_ACCT=laov1.liv1.testnet && \
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
near call $NEAR_ACCT add_item '{"item_id": "item_123", "direct": 123,"email":123, "fb":123,"g_search":123,"organic":123,"youtube":123}' --accountId liv1.testnet
echo "!!! call get_item [item]" && \
near call $NEAR_ACCT get_item '{"item_id": "item_123"}' --accountId liv1.testnet