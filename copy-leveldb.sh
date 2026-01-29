#!/usr/bin/env bash
set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LEVELDB_DEST="$PROJECT_DIR/leveldb"

# Detect OS and set source path
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    SRC="$HOME/.config/Element/Local Storage/leveldb"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    SRC="$HOME/Library/Application Support/Element/Local Storage/leveldb"
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "cygwin" ]] || [[ "$OSTYPE" == "win32" ]]; then
    SRC="$APPDATA/Element/Local Storage/leveldb"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

if [ ! -d "$SRC" ]; then
    echo "Error: Element LevelDB not found at $SRC"
    exit 1
fi


if [ -d "$LEVELDB_DEST" ]; then
    rm -rf "$LEVELDB_DEST"
fi

cp -r "$SRC" "$LEVELDB_DEST"
echo "✓ LevelDB copied to $LEVELDB_DEST"


