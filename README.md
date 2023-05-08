# GraphQL(Juniper) & Rust based Crypto Server

This is a Rust code example that demonstrates how to use the Juniper and Warp libraries to create a GraphQL server.

## How to run

Clone the repository: git clone https://github.com/username/juniper-warp-example.git  
Go to the project directory: cd crypto_server_rust_graphql  
Install dependencies: cargo install --path .  
Start the server: cargo run  
Open http://localhost:8080/graphiql in your browser to access the GraphiQL IDE.  

## Test with curl

curl -X POST -H "Content-Type: application/json" --data '{ "query": "{ currencies { id fullName ask bid last open low high feeCurrency } }" }' http://localhost:8080/graphql

curl -X POST -H "Content-Type: application/json" --data '{ "query": "{ currency(symbol: \"ETHBTC\") { id fullName ask bid last open low high feeCurrency } }" }' http://localhost:8080/graphql


## Code structure

src/main.rs: The main Rust file that sets up the server and defines the GraphQL schema using Juniper.  
src/schema.rs: The Rust file that defines the GraphQL types and resolvers using Juniper.  
src/hitbtc.rs: The Rust file that defines functions to fetch cryptocurrency data from the HitBTC API.  

## Libraries used

**Juniper:** A GraphQL server library for Rust.  
**Warp:** A fast, simple and lightweight web framework for Rust.  
**Reqwest:** A simple HTTP client for Rust.  
**Serde:** A serialization/deserialization library for Rust.  
**Serde_json:** A JSON serialization/deserialization library for Rust.  

## License

This code is licensed under the MIT License.  