#!/bin/bash
geckodriver_path=$(whereis geckodriver | cut -d ' ' -f 2)
$geckodriver_path &

GECKO_PID=$!

cd /usr/src/app
./target/release/gisem_presencas $1 $2

kill $GECKO_PID
