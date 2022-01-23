VERSION="v0.2.0"
TARGET=./sdsl-rs/src/backend/sdsl_c/sdsl-c-template.zip
if [ ! -f "$TARGET" ]; then
    wget -O $TARGET https://github.com/sdsl-rs/sdsl-c-template/releases/download/${VERSION}/sdsl-c-template-${VERSION}.zip
fi