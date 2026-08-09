#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO_TOML="${ROOT_DIR}/Cargo.toml"
README_MD="${ROOT_DIR}/README.md"

usage() {
    cat <<'EOF'
Usage: ./scripts/manage.sh <command>

Commands:
  clean    Remove build artifacts
  build    Compile debug binary
  run      Run interactive terminal demo
  release  Build optimized binary
  version  Display current version
  bump     Increment patch version (x.y.z -> x.y.z+1)
  minor    Increment minor version (x.y.z -> x.(y+1).0)
  major    Increment major version ((x+1).0.0)
  lint     Run make lint
  test     Run make test
  format   Run make format
  docs     Run make docs
EOF
}

get_version() {
    sed -nE 's/^version = "([0-9]+\.[0-9]+\.[0-9]+)"/\1/p' "${CARGO_TOML}" | head -n1
}

set_version() {
    local new_version="$1"
    sed -i -E "0,/^version = \"[0-9]+\.[0-9]+\.[0-9]+\"/s//version = \"${new_version}\"/" "${CARGO_TOML}"
    sed -i -E "s|(https://img.shields.io/badge/version-)[0-9]+\.[0-9]+\.[0-9]+(-blue\.svg)|\1${new_version}\2|g" "${README_MD}"
    sed -i -E "s|^Current version: \`[0-9]+\.[0-9]+\.[0-9]+\`|Current version: \`${new_version}\`|g" "${README_MD}"
}

bump_version() {
    local mode="$1"
    local version
    local major
    local minor
    local patch
    IFS='.' read -r major minor patch <<<"$(get_version)"

    case "${mode}" in
    patch)
        patch=$((patch + 1))
        ;;
    minor)
        minor=$((minor + 1))
        patch=0
        ;;
    major)
        major=$((major + 1))
        minor=0
        patch=0
        ;;
    esac

    version="${major}.${minor}.${patch}"
    set_version "${version}"
    echo "${version}"
}

main() {
    local command="${1:-}"
    cd "${ROOT_DIR}"
    case "${command}" in
    clean)
        cargo clean
        ;;
    build)
        cargo build
        ;;
    run)
        cargo run
        ;;
    release)
        cargo build --release
        ;;
    version)
        get_version
        ;;
    bump)
        bump_version patch
        ;;
    minor)
        bump_version minor
        ;;
    major)
        bump_version major
        ;;
    lint | test | format | docs)
        make "${command}"
        ;;
    *)
        usage
        exit 1
        ;;
    esac
}

main "${@}"
