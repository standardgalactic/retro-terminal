#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "${ROOT_DIR}"

backup_cargo="$(mktemp)"
backup_readme="$(mktemp)"
cp "${ROOT_DIR}/Cargo.toml" "${backup_cargo}"
cp "${ROOT_DIR}/README.md" "${backup_readme}"

cleanup() {
    cp "${backup_cargo}" "${ROOT_DIR}/Cargo.toml"
    cp "${backup_readme}" "${ROOT_DIR}/README.md"
    rm -f "${backup_cargo}" "${backup_readme}"
}

trap cleanup EXIT

assert_eq() {
    local actual="$1"
    local expected="$2"
    local message="$3"
    if [[ "${actual}" != "${expected}" ]]; then
        echo "Assertion failed: ${message}" >&2
        echo "Expected: ${expected}" >&2
        echo "Actual:   ${actual}" >&2
        exit 1
    fi
}

current_version="$(./scripts/manage.sh version)"
cargo_version="$(sed -nE 's/^version = "([0-9]+\.[0-9]+\.[0-9]+)"/\1/p' Cargo.toml | head -n1)"
assert_eq "${current_version}" "${cargo_version}" "version command should match Cargo.toml"

IFS='.' read -r major minor patch <<<"${current_version}"
expected_patch="${major}.${minor}.$((patch + 1))"

bumped_version="$(./scripts/manage.sh bump)"
assert_eq "${bumped_version}" "${expected_patch}" "bump should increment patch version"

grep -q "^version = \"${expected_patch}\"$" Cargo.toml
grep -q "version-${expected_patch}-blue.svg" README.md
grep -q "^Current version: \`${expected_patch}\`$" README.md

echo "manage.sh tests passed"
