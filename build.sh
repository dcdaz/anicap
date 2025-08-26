#!/bin/sh

# Author : Daniel Cordova
# Github : @dcdaz

# Script that build and compress the whole Anicap application

TARGET="$1"
SCRIPT_PATH=$(dirname "$0")


# Delete Current compressed file
[ -e $SCRIPT_PATH/deployment.tar.gz ] && rm -- $SCRIPT_PATH/deployment.tar.gz

# Copy database
cp -r $SCRIPT_PATH/database/ $SCRIPT_PATH/deployment/

# Build backend and copy
mkdir $SCRIPT_PATH/deployment/backend
if [ -z $TARGET ]; then
    cargo build --release
    wait
    cp -r $SCRIPT_PATH/target/release/anicap $SCRIPT_PATH/deployment/backend/
else
    cargo build --release --target $TARGET
    wait
    cp -r $SCRIPT_PATH/target/$TARGET/release/anicap $SCRIPT_PATH/deployment/backend/
fi

# Build frontend and copy
cd "$SCRIPT_PATH/frontend"
pnpm run build
wait
cd ..
cp -r $SCRIPT_PATH/frontend/dist/* deployment/

# Compress the whole deployment folder
tar -cvzf deployment.tar.gz deployment/
wait

# Clean up deployment folder
rm -r $SCRIPT_PATH/deployment/assets
rm -r $SCRIPT_PATH/deployment/backend
rm -r $SCRIPT_PATH/deployment/database
rm -r $SCRIPT_PATH/deployment/icon.svg
rm -r $SCRIPT_PATH/deployment/index.html

echo "-------------------------------------"
echo "Deployment package built successfully"
echo "-------------------------------------"