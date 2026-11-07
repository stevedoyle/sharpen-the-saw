#!/bin/sh
# End-to-end checks for pcq-demo.
#
#   usage: demo_pipeline_test.sh <path-to-pcq-demo> <case>
#
# Each case writes the demo's output to a file instead of piping it, so a
# non-zero exit from pcq-demo fails the test (set -e) rather than being
# hidden behind the last command of a pipeline.

set -eu

if [ $# -ne 2 ]; then
    echo "usage: $0 <pcq-demo> <case>" >&2
    exit 2
fi

demo=$1
case_name=$2

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

fail() {
    echo "FAIL ($case_name): $*" >&2
    exit 1
}

case $case_name in
every-line-once)
    # Every input line comes out exactly once; sorting by seq restores input.
    seq 1 5000 > "$tmp/in"
    "$demo" 8 < "$tmp/in" > "$tmp/out"
    sort -n "$tmp/out" | cut -f2- > "$tmp/restored"
    cmp -s "$tmp/in" "$tmp/restored" || fail "output does not match input"
    ;;
seq-matches-line)
    # The seq prefix is the input line number (input line N is "N"), and
    # there are exactly 1000 lines, so empty output cannot pass.
    seq 1 1000 > "$tmp/in"
    "$demo" 4 < "$tmp/in" > "$tmp/out"
    awk -F'\t' '$1 != $2 { bad = 1 } END { exit (bad || NR != 1000) }' \
        "$tmp/out" || fail "seq/line mismatch or wrong line count"
    ;;
empty-input)
    "$demo" 3 < /dev/null > "$tmp/out"
    [ ! -s "$tmp/out" ] || fail "expected no output for empty input"
    ;;
rejects-bad-usage)
    for arg in 0 -1 4x '' 1025 99999; do
        if "$demo" "$arg" < /dev/null > /dev/null 2>&1; then
            fail "accepted consumer count '$arg'"
        fi
    done
    if "$demo" 1 2 < /dev/null > /dev/null 2>&1; then
        fail "accepted extra arguments"
    fi
    ;;
*)
    echo "unknown case: $case_name" >&2
    exit 2
    ;;
esac
