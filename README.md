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
for now. See usage examples below.

As for the server, it only supports single user queries and setting non-avatar
fields. It breaks the spec in lots of ways, but maybe someday it won't. However,
the server does support auth via `argon2` and TLS. You still shouldn't use this
code for anything important.

To run the server:

 1) Install [Rust](https://rustup.rs/).
 2) Run with `cargo run -p femur-server -- --help` to see the help text.
 3) Either run with `--no-tls` or supply a TLS keypair according to the help
    text.

The server listens on `0.0.0.0:443` by default. If you pass `--no-tls`, then the
default is `0.0.0.0:80`. You may also pass `--listen <ADDRESS>` to listen on
some other address and port, such as `--listen localhost:23856`.

Note that the server stores statuses and the auth database in text files in the
directory where you run it. Eventually, I intend to make this configurable also.

To interact with the server:

```
$ curl localhost:23856/fmrl/users/alice
{"avatar":null,"name":null,"status":"hello, femur!","emoji":null,"media":null,"media_type":null}
```

For better formatting, try Python:

```
$ curl -s localhost:23856/fmrl/users/alice | python3 -m json.tool
{
    "avatar": null,
    "name": null,
    "status": "hello, femur!",
    "emoji": null,
    "media": null,
    "media_type": null
}
```

To add a user:

```
$ echo "username $(echo -n "password" | argon2 salt-goes-here -e)" >> auth-db
```

To set your status:

```
$ curl localhost:23856/fmrl/users/bob -X PUT -d '{"status": "updated status"}' \
    --header "Authorization: Basic $(echo -n "bob:password" | base64)"
$ curl -s localhost:23856/fmrl/users/bob | python3 -m json.tool
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
