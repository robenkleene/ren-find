#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

diff --unified find.txt <(sed s/changes/altered/g find.txt) > patch.patch || true

sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

rm patch.patch.bak

# Delete

diff --unified find.txt <(printf "") > delete.patch || true

sed -i.bak '1s/.*/--- original/' delete.patch
sed -i.bak '2s/.*/+++ modified/' delete.patch

rm delete.patch.bak
