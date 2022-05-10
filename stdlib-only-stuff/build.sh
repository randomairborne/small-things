#!/bin/bash
rustc --edition 2021 -C lto=fat -C panic=abort -C codegen-units=1 -C opt-level=z -C debuginfo=0 -C strip="symbols"  -C overflow-checks=on $1
echo Built $1
