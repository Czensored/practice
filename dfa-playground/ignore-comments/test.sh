#!/usr/bin/env bash
set -u
fails=0

for i in $(seq 1 6); do
    f="programming_assignment_1-test_file_${i}.c"
    echo "== TEST $i: $f =="
    ./strip.out "$f"
    rc=$?

    exp=$([ "$i" -le 4 ] && echo 0 || echo 1)
    if [ "$rc" -ne "$exp" ]; then
        echo "!! unexpected $([ "$rc" -eq 0 ] && echo PASS || echo FAIL)"
        fails=$((fails + 1))
    fi

    if [ "$i" -le 4 ]; then
        echo "== DIFF $i =="
        if ! diff -u \
            "programming_assignment_1-test_file_${i}-comments_replaced_with_whitespace.c" \
            "programming_assignment_1-test_file_${i}-comments_replaced_with_whitespace_GIVEN.c"; then
            fails=$((fails + 1))
        fi
    fi
done

if [ "$fails" -eq 0 ]; then
    printf "\nAll tests passed\n"
    exit 0
else
    printf "\n%d tests failed\n" "$fails"
    exit 1
fi
