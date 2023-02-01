# Federation Example

An example of using [Apollo Federation](https://www.apollographql.com/docs/federation/) to compose GraphQL services into a single data graph.

## The schema

You can view the full schema in [Apollo Studio](https://studio.apollographql.com/public/async-graphql-Examples/home?variant=current) without needing to run the example (you will need to run the example in order to query it).

## How to run

1. Install [Rover](https://www.apollographql.com/docs/rover/)
2. Run `/start.sh` which will:
   1. Start each subgraph with `cargo run --bin {subgraph_name}`
   2. Add each subgraph to `rover dev` with `rover dev --url http://localhost:{port} --name {subgraph_name}`
3. Visit `http://localhost:3000` in a browser.
4. You can now run queries like the one in `query.graphql` against the router.
