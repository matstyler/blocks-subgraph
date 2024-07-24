# Blocks Subgraph

###### Substreams-powered

Simple and useful - blocks subgraph for your project.

## Prerequisites

This [requires the dependencies necessary for local Substreams development](https://substreams.streamingfast.io/developers-guide/installation-requirements).

## Requirements

1. Rust - https://rustup.rs/

2. Graph-cli - https://thegraph.com/docs/en/quick-start/#2-install-the-graph-cli

3. substreams-cli - https://substreams.streamingfast.io/getting-started/installing-the-cli

## Local development

```
yarn install # install graph-cli
yarn substreams:prepare # build and package the substreams module
yarn subgraph:build # build the subgraph
yarn deploy # deploy the subgraph
```

## Deployments

##### Mainnet: https://api.studio.thegraph.com/query/58996/mainnet-blocks-subgraph/version/latest

##### Aribtrum: https://api.studio.thegraph.com/query/58996/arbitrum-blocks-subgraph/version/latest

##### Optimism: https://api.studio.thegraph.com/query/58996/optimism-blocks-subgraph/version/latest

##### Base: Soon

###### Ethereum Sepolia: https://api.studio.thegraph.com/query/58996/sepolia-blocks-subgraph/version/latest

## Example queries

#### Fetch 5 latest blocks with details

```graphql
{
    blocks(first: 5, orderBy: number, orderDirection: desc) {
        id
        number
        timestamp

        size
        transactions

        hash
        gas_used
    }
}
```

#### Get block by timestamp

```graphql
{
    blocks(where: { timestamp: 1438292598 }) {
        id
        number
        timestamp

        size
        transactions

        hash
        gas_used
    }
}
```