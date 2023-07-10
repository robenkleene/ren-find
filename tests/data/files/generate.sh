#!/usr/bin/env bash

set -euo pipefail

cd "$(dirname "$0")" || exit 1

diff --unified start.txt <(sed s/changes/altered/g start.txt) > patch.patch
