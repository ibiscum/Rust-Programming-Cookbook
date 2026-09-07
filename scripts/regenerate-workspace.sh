#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"

members=()
declare -A seen_package_names=()
while IFS= read -r manifest; do
  if grep -qE '^\[package\]' "$manifest"; then
    package_name="$(
      sed -n '/^\[package\]/,/^\[/{
        /^\[package\]/d
        /^\[/d
        p
      }' "$manifest" \
      | sed -n 's/^name[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' \
      | head -n 1
    )"

    if [[ -z "${package_name}" ]]; then
      continue
    fi

    if [[ -n "${seen_package_names[$package_name]:-}" ]]; then
      printf 'Skipping duplicate package name "%s": %s (already have %s)\n' \
        "$package_name" "${manifest#./}" "${seen_package_names[$package_name]}" >&2
      continue
    fi

    seen_package_names[$package_name]="${manifest#./}"
    members+=("${manifest#./}")
  fi
done < <(find . -name Cargo.toml -not -path './Cargo.toml' | sort)

for i in "${!members[@]}"; do
  members[$i]="${members[$i]%/Cargo.toml}"
done

default_members=()
while IFS= read -r main_file; do
  default_members+=("${main_file#./}")
done < <(find . -path '*/src/main.rs' | sort)

for i in "${!default_members[@]}"; do
  default_members[$i]="${default_members[$i]%/src/main.rs}"
done

{
  echo '[workspace]'
  echo 'resolver = "2"'
  echo 'members = ['
  for member in "${members[@]}"; do
    printf '  "%s",\n' "$member"
  done
  echo ']'
  echo 'default-members = ['
  for member in "${default_members[@]}"; do
    printf '  "%s",\n' "$member"
  done
  echo ']'
} > Cargo.toml

echo "Updated $repo_root/Cargo.toml"