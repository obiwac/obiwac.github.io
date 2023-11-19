# [obiw.ac](https://obiw.ac)

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

Because GitHub Pages (annoyingly) does not allow us a wide range of options for selecting which directory to deploy our website from, we must use [this](https://gist.github.com/cobyism/4730490) trick.
The gist of it is that we create a subtree of `static` and then push it to the `gh-pages` branch:

```console
git push origin $(git subtree split --prefix static):gh-pages -f
```

That's all folks!
