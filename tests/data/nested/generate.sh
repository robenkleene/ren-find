#!/usr/bin/env bash

set -euo pipefail

sorted=$(awk '{ print length, $0 }' < find.txt | sort -nsr | cut -d" " -f2-)
diff --unified <(echo "$sorted") <(echo "$sorted" | sed 's/\(.*\)changes/\1altered/') > patch.patch || true

sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

rm patch.patch.bak
