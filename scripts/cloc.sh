set -e
find ./ -name "*.rs" -print0 | xargs -0 cat | wc -l