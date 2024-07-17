#!/bin/bash

source .env

cargo stylus deploy --verbose \
    --endpoint https://rpc.open-campus-codex.gelato.digital \
    --private-key $PRIV_KEY
