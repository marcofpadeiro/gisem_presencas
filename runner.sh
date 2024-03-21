#!/bin/bash
geckodriver &
GECKO_PID=$!

./target/release/gisem_rust $1 $2

kill $GECKO_PID
