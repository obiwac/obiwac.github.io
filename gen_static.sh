#!/bin/sh
set -xe

ROUTES=$(cat <<-END
/
/mcpy
/moodle
/gdpr
END
)

rm -rf static
cp -r public static

cargo run &
sleep 1 # XXX should be enough, lol

for route in $ROUTES; do
	mkdir -p static$route
	fetch http://localhost:8000$route -o static$route/index.html
done

kill $(jobs -p)
