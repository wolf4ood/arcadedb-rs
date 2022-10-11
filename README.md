<h1 align="center">arcadedb-rs</h1>

<div align="center">
 <strong>
   A Rust Client for ArcadeDB 
 </strong>
</div>


<br />

<div align="center">
  <a href="https://github.com/wolf4ood/arcadedb-rs/actions?query=workflow%3ATests">
    <img src="https://github.com/wolf4ood/arcadedb-rs/workflows/Tests/badge.svg"
    alt="Tests status" />
  </a>
  
  <a href="https://coveralls.io/github/wolf4ood/arcadedb-rs?branch=master">
    <img src="https://coveralls.io/repos/github/wolf4ood/arcadedb-rs/badge.svg?branch=master"
    alt="Coverage status" />
  </a>
  <a href="https://crates.io/crates/arcadedb-client">
    <img src="https://img.shields.io/crates/d/arcadedb-client.svg?style=flat-square"
      alt="Download" />
  </a>
  <a href="https://docs.rs/arcadedb-client">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>

   <a href="https://opensource.org/licenses/Apache-2.0">
    <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg"
      alt="license" />
  </a>

   <a href="https://deps.rs/repo/github/wolf4ood/arcadedb-rs">
    <img src="https://deps.rs/repo/github/wolf4ood/arcadedb-rs/status.svg"
      alt="license" />
  </a>


  
</div>

# Getting Started


## Installation


Install from [crates.io](https://crates.io/)

```toml
[dependencies]
arcadedb-rs = "*"
```


## Examples



# Development

## Compiling


```
git clone https://github.com/wolf4ood/arcadedb-rs.git
cd arcadedb-rs
cargo build
```


## Running Tests


You can use docker-compose to start an instance for testing. Use the env variable `ARCADEDB_SERVER`
in order to specify the version of ArcadeDB

```
cd docker-compose
export ARCADEDB_SERVER=22.8.1
docker-compose up -d
cd ..
cargo test
```
