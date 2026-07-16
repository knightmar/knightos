#!/usr/bin/env bash
set -euo pipefail

cd /workspace
./build.sh

if [ ! -f /workspace/knightos.iso ]; then
    echo "Erreur : knightos.iso introuvable après le build." >&2
    exit 1
fi

mkdir -p /output
cp /workspace/knightos.iso /output/knightos.iso
echo "ISO copié dans /output/knightos.iso"
