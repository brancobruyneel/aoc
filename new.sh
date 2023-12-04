#!/bin/bash

set -e

YEAR=$(date "+%Y")
MONTH=$(date "+%-m")
DAY=$(date "+%d")

if [ $MONTH -ne 12 ]; then
	echo "You're too early, idiot"
	exit 1
fi

if ! grep -Eo "^session=[a-z0-9]+$" ./.cookie >/dev/null 2>&1; then
	echo "No session cookie found"
	exit 1
fi

DIR="$YEAR/rust/src/bin/$DAY"

mkdir -p $DIR

cd $DIR

cargo init -q

DAY=$(date "+%-d")

curl -b "$(cat ../../.cookie)" -s "https://adventofcode.com/$YEAR/day/$DAY/input" >input.txt

cp ../../template/main.rs ./src
