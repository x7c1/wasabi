#!/usr/bin/env bash

. ./builder/setup-env.sh
cd "$PROJECT_ROOT" || exit 1

cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_X86}"

cargo build \
  --verbose \
  ${BUILD_MODE} \
  --target="${TARGET_ARM}"

cargo test \
  ${BUILD_MODE} \
  --target="${TARGET_X86}" \
  -- --nocapture
