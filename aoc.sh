#!/bin/bash

# Advent of Code helper commands

ACTION="$1"
DAY="$2"

TEMPLATE=".day_template.rs"
INPUT_TEMPLATE=".day_input_template.rs"

EXTERNS="--extern ndarray=deps/libndarray-4276025192ebc488.rlib"

if [[ "$ACTION" == "create" ]]; then
	TOPIC="$3"
	if [[ -z "$TOPIC" ]]; then
		echo "No topic provided."
		exit 1
	fi
	if [ -f "day${DAY}.rs" ]; then
		echo "Day already exists."
		exit 1
	else
		cp $TEMPLATE "day${DAY}.rs"
		cp $INPUT_TEMPLATE "day${DAY}_input.rs"

		sed -i -s -r "s/\\{\\{day\\}\\}/${DAY}/" "day${DAY}.rs" "day${DAY}_input.rs"
		sed -i -s -r "s/\\{\\{topic\\}\\}/${TOPIC}/" "day${DAY}.rs" "day${DAY}_input.rs"

		echo "Created files for day ${DAY} - ${TOPIC}."
	fi

elif [[ "$ACTION" == "run" ]]; then
	cargo build && ./target/debug/aoc2021 ${DAY}
else
	echo "Unknown action ('$ACTION')."
	exit 1
fi
