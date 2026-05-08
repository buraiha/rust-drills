#!/usr/bin/env bash
set -euo pipefail

IMAGE="${IMAGE:-rust-drills}"
CARGO_VOL="${CARGO_VOL:-rust-drills-cargo}"

docker run --rm -it \
  -v "$(pwd)":/work \
  -v "${CARGO_VOL}":/cargo \
  "${IMAGE}" \
  "$@"
