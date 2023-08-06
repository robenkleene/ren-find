#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

sorted=$(awk '{ print length, $0 }' < find.txt | sort -nsr | cut -d" " -f2-)
diff --unified <(echo "$sorted") <(echo "$sorted" | sed 's/\(.*\)changes/\1altered/') > patch.patch || true

sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

# newline messages
line_fix='6i\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch
line_fix='$a\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" patch.patch

rm patch.patch.bak

# Delete

diff --unified <(echo "$sorted") <(printf "") > delete.patch || true

sed -i.bak '1s/.*/--- original/' delete.patch
sed -i.bak '2s/.*/+++ modified/' delete.patch
line_fix='$a\
\\ No newline at end of file
'
sed -i.bak "${line_fix}" delete.patch

rm delete.patch.bak
