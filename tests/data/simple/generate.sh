#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

diff --unified start.txt <(sed s/changes/altered/g start.txt) > patch.patch || true
sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

sed -i.bak '3s#.*#@@ -1,5 +1,5 @@#' patch.patch
sed -i.bak '12d' patch.patch

rm patch.patch.bak

