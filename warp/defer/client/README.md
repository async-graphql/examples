## warp-defer-client

## Setting the project up locally

First of all make sure you are using node `12.13.1` (any node 12.x would also do) and latest yarn, you can always have a look at the `engines` section of the `package.json`.

```sh
$ yarn (install)
$ yarn dev
```

After doing this, you'll have a server with hot-reloading running at [http://localhost:3004](http://localhost:3004)

## When changing the graphql server schema

```sh
$ yarn update-graphql
```
