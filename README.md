# obiwac.github.io

Super simple static website about myself ;)

This uses [Rocket.rs](https://rocket.rs) combined with [maud](https://maud.lambda.xyz) (to get away from the god-awful language known as HTML).

To start the webserver, run:

```console
cargo run
```

To export a static site, run:

```console
sh gen_static.sh
```

It's a bit of a hacky script, so you may have to run it a couple times before it works.

That's all folks!
