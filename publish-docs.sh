#!/bin/bash
set -e

cargo doc --no-deps -p c-types
echo "<meta http-equiv=refresh content=0;url=c_types/index.html>" > target/doc/index.html

git config user.name "Travis CI"
pip install --user ghp-import
~/.local/bin/ghp-import -n target/doc

openssl aes-256-cbc -K "$encrypted_7f3527787a94_key" -iv "$encrypted_7f3527787a94_iv" -in publish-key.enc -out ~/.ssh/publish-key -d
chmod u=rw,og= ~/.ssh/publish-key
echo "Host github.com" >> ~/.ssh/config
echo "  IdentityFile ~/.ssh/publish-key" >> ~/.ssh/config
git remote set-url origin git@github.com:dimbleby/rust-c-types
git push origin +gh-pages
