#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DOCKERFILE="${DOCKERFILE:-containerization/Dockerfile}"
PLATFORM="${PLATFORM:-linux/amd64}"
IMAGE_NAME="${IMAGE_NAME:-fraud-detection}"
TAG="${TAG:-$(git -C "$ROOT_DIR" rev-parse --short HEAD)}"
GHCR_OWNER="${GHCR_OWNER:-ismaellima4}"

IMAGE_REF="ghcr.io/${GHCR_OWNER}/${IMAGE_NAME}:${TAG}"
LATEST_REF="ghcr.io/${GHCR_OWNER}/${IMAGE_NAME}:latest"

echo "Building ${IMAGE_REF}..."
docker buildx build \
  --platform "${PLATFORM}" \
  -f "${ROOT_DIR}/${DOCKERFILE}" \
  -t "${IMAGE_REF}" \
  -t "${LATEST_REF}" \
  --push \
  "${ROOT_DIR}"

echo "Done: ${IMAGE_REF} and ${LATEST_REF}"
