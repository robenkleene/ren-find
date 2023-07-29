#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

sorted=$(awk '{ print length, $0 }' < find.txt | sort -n -s | cut -d" " -f2-)
diff --unified <(echo "$sorted") <(echo "$sorted" | sed 's/\(.*\)changes/\1altered/') > patch.patch || true
sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

rm patch.patch.bak
echo "\ No newline at end of file" >> patch.patch
