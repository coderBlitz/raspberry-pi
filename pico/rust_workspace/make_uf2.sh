#!/bin/bash
# TODO: Make this script the runner for cargo (`runner` in .cargo/config.toml)
#        to speed up compiling and loading onto pico.

if [[ $1 == "" ]]; then
	echo "Please give the path to the main binary."
	exit 1
fi

# Stage 2 when experimenting
#STAGE2="target/thumbv6m-none-eabi/release/stage2"

# Known good stage 2 for regular flash XIP.
STAGE2="stage2-normal"

arm-none-eabi-objcopy -O binary -j .text $STAGE2 /tmp/stage2.bin
arm-none-eabi-objcopy -O binary $1 /tmp/main.bin

cp /tmp/stage2.bin /tmp/padded.bin
truncate -s 256 /tmp/padded.bin
cat /tmp/padded.bin /tmp/main.bin > /tmp/combo.bin

./uf2 /tmp/combo.bin /tmp/combo.uf2
