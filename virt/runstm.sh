#!/bin/bash
#make
./qemu-system-arm -machine netduinoplus2 -kernel /home/benni/development/os_dev/NextLevelRTOS/kernel/target/thumbv7em-none-eabihf/debug/kernel -nographic -cpu cortex-m4 -S -s