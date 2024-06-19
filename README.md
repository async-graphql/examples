# Examples for async-graphql

A git submodule that shows example async-graphql projects.


## Directory structure

- [poem] Examples for `poem`
- [actix-web] Examples for `actix-web`
- [warp] Examples for `warp`
- [tide] Examples for `tide`
- [rocket] Examples for `rocket`
- [axum] Examples for `axum`
- [loco] Examples for `loco`
- [federation] Examples for [Apollo Federation](https://www.apollographql.com/docs/federation/)


## Running Examples

To run the examples, clone the top-level repo, [async-graphql](https://github.com/async-graphql/async-graphql) and then issue the following commands:

```bash
git clone async-graphql/async-graphql
# in async-graphql repo, install needed dependencies
cargo build

# update this repo as a git submodule
git submodule update
```

To run the example axum-starwars:
```
# change into the example folder and run a relevant binary
cargo run --bin axum-starwars
```

To list all available binary targets:
```
cargo run --bin
```