import { ApolloServer } from "@apollo/server";

async function main() {
  new ApolloServer();
}

main().catch(console.error);