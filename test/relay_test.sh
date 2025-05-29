#!/usr/bin/env bash

echo "Starting the testing routine!"

TOTAL_RELAYS=18
SMART_RELAY_UTILITY_PATH="/home/user"

echo There are $TOTAL_RELAYS relays mounted on the board

cd $SMART_RELAY_UTILITY_PATH

#for (( number=1; number <= $TOTAL_RELAYS; number++ )
for (( number=1; number <= $TOTAL_RELAYS; number++ ))
do
echo Relay $number is ON
./smart_relay set-state $number on
sleep 1
echo Relay $number is OFF
./smart_relay set-state $number off
sleep 1
done
