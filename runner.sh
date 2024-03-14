#!/bin/bash

sleep=5 # minutes

while true; do 
    echo "Running script"
    ./target/release/gisem_rust

    echo "Sleeping for $sleep minutes"
    sleep $((60*$sleep))
done
