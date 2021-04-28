VERSION="v0.1.0"
TARGET=./sdsl-rs/src/backend/sdsl_c/sdsl-c-template.zip
if [ ! -f "$TARGET" ]; then
    wget -O $TARGET https://github.com/sdsl-rs/sdsl-c-template/archive/refs/tags/${VERSION}.zip
fi