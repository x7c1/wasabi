#!/usr/bin/env bash

. ./builder/setup-env.sh
. ./builder/lib.linux_x86.sh
cd "$PROJECT_ROOT" || exit 1

run_unit_tests_for_linux_x86

# usage:
# $ ./scripts/run_tests.sh
