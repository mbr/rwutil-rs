#!/bin/sh
set -e

rm -rf target/doc
cargo doc -p rwutil
gittar -b gh-pages file:target/doc/*
git push origin gh-pages
