#!/bin/bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
TMP_DIR="$(mktemp -d /tmp/rsa-demo.XXXXXX)"
OUTPUT_FILE="$TMP_DIR/output.txt"
PUBLIC_PEM="$TMP_DIR/public.pem"
PRIVATE_PEM="$TMP_DIR/private.pem"
BROKEN_PUBLIC_PEM="$TMP_DIR/public-broken.pem"

cleanup() {
    rm -rf "$TMP_DIR"
}

trap cleanup EXIT

echo "Running RSA demo..."
echo

CARGO_TARGET_DIR=/tmp/rsa-encryption-target cargo run --release --manifest-path "$ROOT_DIR/Cargo.toml" > "$OUTPUT_FILE"

echo "Program output:"
echo "----------------------------------------"
cat "$OUTPUT_FILE"
echo "----------------------------------------"
echo

awk '/-----BEGIN RSA PUBLIC KEY-----/{flag=1} flag{print} /-----END RSA PUBLIC KEY-----/{flag=0}' "$OUTPUT_FILE" > "$PUBLIC_PEM"
awk '/-----BEGIN RSA PRIVATE KEY-----/{flag=1} flag{print} /-----END RSA PRIVATE KEY-----/{flag=0}' "$OUTPUT_FILE" > "$PRIVATE_PEM"

echo "Validating exported public key with OpenSSL..."
openssl rsa -RSAPublicKey_in -pubin -in "$PUBLIC_PEM" -text -noout
echo

echo "Validating exported private key with OpenSSL..."
openssl rsa -in "$PRIVATE_PEM" -text -noout
echo


echo "Corrupting one character in the public key and rerunning OpenSSL..."
perl -pe 'BEGIN{$done=0} unless($done){$done=1 if s/A/B/}' "$PUBLIC_PEM" > "$BROKEN_PUBLIC_PEM"
openssl rsa -RSAPublicKey_in -pubin -in "$BROKEN_PUBLIC_PEM" -text -noout


# When you do openssl, it should comment some more information on what the encryption method is
# Public and private key to be installed in the ssl
