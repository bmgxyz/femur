`femur` is an **incomplete** implementation of the [`fmrl`][fmrl] protocol
(pronouced "ephemeral"). It's a simple client-server protocol where users read
and set status messages, along with some metadata. There are no feeds, only
statuses. It's basically `finger` with structured data. Cool, right?

This software comes in three parts:

 - `femur-server`, an HTTP server that exposes an API according to the spec (in
    principle, see below)
 - `femur-client`, a client that gathers statuses for the user and lets them set
    their own
 - `femur`, the library that contains common data structures and logic between
    the client and server

Currently the client doesn't do anything; you're better off just using `curl`
for now.

As for the server, it only supports single user queries and setting non-avatar
fields. It breaks the spec in lots of ways, but maybe someday it won't. Notably,
the server **does not support TLS or auth yet so anyone can set anyone else's
status**. Don't use this code for anything important. These are the next areas I
want to work on.

To run the server:

 1) Install [Rust](https://rustup.rs/).
 2) Run with `cargo run -p femur-server`.

The server is hard-coded to bind on `0.0.0.0:23856`. In the future I intend to
make it possible to configure this without recompiling.

Note that the server stores statuses in text files in the directory where you
run it. Eventually, I intend to make this configurable also.

To interact with the server:

```
$ curl localhost:23856/fmrl/users/@alice@localhost
{"avatar":null,"name":null,"status":"hello, femur!","emoji":null,"media":null,"media_type":null}
```

For better formatting, try Python:

```
$ curl -s localhost:23856/fmrl/users/@bradley@localhost | python3 -m json.tool
{
    "avatar": null,
    "name": null,
    "status": "hello, femur!",
    "emoji": null,
    "media": null,
    "media_type": null
}
```

To set your status:

```
$ curl localhost:23856/fmrl/users/@bradley@localhost -X PUT -d '{"status": "updated status"}'
$ curl -s localhost:23856/fmrl/users/@bradley@localhost | python3 -m json.tool
{
    "avatar": null,
    "name": null,
    "status": "updated status",
    "emoji": null,
    "media": null,
    "media_type": null
}
```

When you find bugs, open an issue (or better yet, open a PR).

[fmrl]: https://github.com/makeworld-the-better-one/fmrl
