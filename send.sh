#!/bin/bash
# for i in {0..10}
# do
#    echo "go.."
#    dd if=./data of=/tmp/s9.in iflag=direct,skip_bytes
# #    sleep 1
#    dd if=./prepare of=/tmp/s9.in iflag=direct,skip_bytes
# #    sleep 1
#    dd if=./task of=/tmp/s9.in iflag=direct,skip_bytes
#    sleep 0.1
# done
#dd if=./data of=/tmp/s9.in iflag=direct,skip_bytes
#sleep 1
#dd if=./prepare of=/tmp/s9.in iflag=direct,skip_bytes
#sleep 1
#dd if=./task of=/tmp/s9.in iflag=direct,skip_bytes
#sleep 2
#dd if=./data of=/dev/ttyUSB0 iflag=direct,skip_bytes
printf "\\x01\\x12\\x34\\x56" >> /dev/ttyUSB0 
sync
#sleep 1
dd if=./prepare of=/dev/ttyUSB0 iflag=direct,skip_bytes
sync
#sleep 1
dd if=./task of=/dev/ttyUSB0 iflag=direct,skip_bytes
sync
#sleep 2
