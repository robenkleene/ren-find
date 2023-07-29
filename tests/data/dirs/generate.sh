#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

diff --unified <(awk '{ print length, $0 }' < find.txt | sort -n -s | cut -d" " -f2-) <(sed s/changes/altered/g find.txt) > patch.patch || true
sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

rm patch.patch.bak
echo "\ No newline at end of file" >> patch.patch
