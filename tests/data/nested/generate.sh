#!/usr/bin/env bash

set -euo pipefail

diff --unified find.txt <(sed 's/\(.*\)change/\1altered/' find.txt) > patch.patch || true

sed -i.bak '1s/.*/--- original/' patch.patch
sed -i.bak '2s/.*/+++ modified/' patch.patch

rm patch.patch.bak
