import { ApolloServer } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';

import resolvers from './resolvers';
import typeDefs from './typeDefs';

async function main(): Promise<void> {
  const server = new ApolloServer({
    typeDefs,
    resolvers,
  });
  const { url } = await startStandaloneServer(server, { listen: { port: 4444 } });
  console.info(`Server listening at ${url}...`);
}

main().catch(console.error);
