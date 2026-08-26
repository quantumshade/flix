#!/usr/bin/env bash

THIS_FOLDER="$(dirname "$0")"

if grep -R -nE '^[[:space:]]*flix add' "$THIS_FOLDER"; then
	echo "Direct flix invocation detected"
	exit 1
fi


rm -f flix.db || exit 1
flix init || exit 1


declare -a failures=()

run() {
	local output
	if ! output=$("$@" 2>&1)
	then
		failures+=("=> $*
$output")
	fi
}

for f in "$THIS_FOLDER/collections/"*.sh
do
	# shellcheck source=/dev/null
	source "$f"
done
for f in "$THIS_FOLDER/movies/"*.sh
do
	# shellcheck source=/dev/null
	source "$f"
done
for f in "$THIS_FOLDER/shows/"*.sh
do
	# shellcheck source=/dev/null
	source "$f"
done

if ((${#failures[@]} > 0)); then
	echo
	echo "The following ${#failures[@]} commands failed:"
	printf '  %s\n' "${failures[@]}"
	exit 1
fi
echo "All commands succeeded"
