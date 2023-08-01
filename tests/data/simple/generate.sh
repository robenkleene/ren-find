#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

diff --unified find.txt <(sed s/changes/altered/g find.txt) > patch.patch || true

sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

# newline messages
line_fix='5i\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch
line_fix='$a\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch

rm patch.patch.bak
