#!/usr/bin/env bash

# stop if non-zero returned.
set -e

# not allow undefined values.
set -u

# enable alias on bash
shopt -s expand_aliases

get_opt_level() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            opt-level=*)
              echo ${OPTARG#*=}
              exit
              ;;
          esac
          ;;
    esac
  done
  echo 2
}

get_build_mode() {
  while getopts ":-:" OPT; do
    case "${OPT}" in
      -)  case "${OPTARG}" in
            debug)
              echo ""
              exit
              ;;
          esac
          ;;
    esac
  done
  echo "--release"
}

is_osx_sdk_installed() {
  target=${OSXCROSS_ROOT}/target/bin/${OSX_SDK_CC}
  if [[ -f ${target} ]]; then
    return 0
  else
    return 1
  fi
}

OPT_LEVEL=$(get_opt_level "$@")
BUILD_MODE=$(get_build_mode "$@")

# defined for this project
export BUILD_MODE
export PROJECT_ROOT="/wasabi"

export OSX_SDK="MacOSX10.15.sdk.tar.bz2"
export OSX_SDK_CC="x86_64-apple-darwin19-clang"
export OSXCROSS_ROOT="${WSB_WORKSPACE}/osxcross"

export TARGET_X86="x86_64-unknown-linux-musl"
export TARGET_ARMV7="armv7-unknown-linux-musleabihf"
export TARGET_MACOS="x86_64-apple-darwin"

export MAX_PARALLEL=$(getconf _NPROCESSORS_ONLN)

# used by rustc
export RUSTFLAGS="-C opt-level=$OPT_LEVEL"

cd ${PROJECT_ROOT}
. ./builder/build-osxcross.sh

if is_osx_sdk_installed; then
  PATH=${OSXCROSS_ROOT}/target/bin:$PATH
fi