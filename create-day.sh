#!/bin/bash

DAYNUM="$1"
DAYTOPIC="$2"

TEMPLATE=".day_template.rs"
INPUT_TEMPLATE=".day_input_template.rs"

cp $TEMPLATE "day${DAYNUM}.rs"
cp $INPUT_TEMPLATE "day${DAYNUM}_input.rs"

sed -i -s -r "s/\\{\\{day\\}\\}/${DAYNUM}/" "day${DAYNUM}.rs" "day${DAYNUM}_input.rs"
sed -i -s -r "s/\\{\\{topic\\}\\}/${DAYTOPIC}/" "day${DAYNUM}.rs" "day${DAYNUM}_input.rs"

