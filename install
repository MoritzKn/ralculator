#!/bin/sh

if test ! -f target/release/ralculator; then
    echo "No release binary found"
    echo "Please run: cargo build --release"
    exit 1
fi

if test ! -d /usr/local/bin/; then
    mkdir -p /usr/local/bin/
fi
cp target/release/ralculator /usr/local/bin/

if test ! -d /usr/local/share/applications/; then
    mkdir -p /usr/local/share/applications/
fi
cp ralculator.desktop /usr/local/share/applications/
