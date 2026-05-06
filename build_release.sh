#!/bin/bash

set -e


dx6 build --platform android --release


# Define source and destination paths
SRC_PATH="/home/glamund/repos/mama_stunden/target/dx/dx_test/release/android/app/app/src/main/jniLibs/arm64-v8a/libdioxusmain.so"
DST_PATH="android/app/src/main/jniLibs/arm64-v8a/libdioxusmain.so"

# Ensure the destination directory exists
mkdir -p "$(dirname "$DST_PATH")"

# Move the file and force overwrite
mv -f "$SRC_PATH" "$DST_PATH"
# end script 

env -C android/ ./gradlew clean
env -C android/ ./gradlew assembleRelease

adb install android/app/build/outputs/apk/release/app-release.apk


adb shell am start -n com.sohnidas_studios.boonk_arbeitszeit/dev.dioxus.main.MainActivity

