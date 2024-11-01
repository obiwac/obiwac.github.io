#!/bin/sh
set -xe

ROUTES=$(cat <<-END
/
/mcpy
/moodle
/gdpr
/bfm
/karwa
/graphic-design
/x-compositing-wm
/24hvelo
/batman
/fprint
/s0ix
END
)

rm -rf static
mkdir -p static
cp -r public static/public
cp CNAME static/CNAME

killall obiwac-website || true
cargo run &
sleep 2 # XXX should be enough, lol

for route in $ROUTES; do
	mkdir -p static$route
	curl http://localhost:8000$route > static$route/index.html
done

kill $(jobs -p)
