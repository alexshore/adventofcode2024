#!/bin/bash

if [[ -z $AOC_COOKIE ]]; then
    echo Please assign your AOC session cookie to AOC_COOKIE
    exit 1
fi

DAY=$(printf %02d $1)

cp -r template day$DAY

sed -i '' "s/DAY/$DAY/g" day$DAY/Cargo.toml

curl --cookie "session=$AOC_COOKIE" https://adventofcode.com/2024/day/$1/input > day$DAY/src/bin/input.txt