#!/bin/sh

VERSION="$1"

if [ -z "$VERSION" ]; then
  echo "Usage: $0 <version>"
  exit 1
fi

update_package_json() {
  FILE="$1"
  echo "Updating $FILE to version $VERSION"

  tmpfile="$(mktemp)"

  jq --arg ver "$VERSION" '
    .version = $ver
    | if .optionalDependencies != null then
        .optionalDependencies |= with_entries(.value = $ver)
      else .
      end
  ' "$FILE" > "$tmpfile" && mv "$tmpfile" "$FILE"

  git add "$FILE"
}

# (2) カレントディレクトリの package.json
if [ -f package.json ]; then
  update_package_json package.json
fi

# (3) サブディレクトリの package.json をループ
for dir in */; do
  [ -f "$dir/package.json" ] && update_package_json "$dir/package.json"
done
