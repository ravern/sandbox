import { ApolloServer, GraphQLRequestContext } from '@apollo/server';
import { startStandaloneServer } from '@apollo/server/standalone';

import resolvers from './resolvers';
import typeDefs from './typeDefs';

const logRequestsPlugin = {
  async requestDidStart(requestContext: GraphQLRequestContext<any>): Promise<void> {
    const name = JSON.stringify(requestContext.request.operationName);
    const variables = JSON.stringify(requestContext.request.variables);
    console.log(`Query ${name} with variables ${variables}`);
  },
};

async function main(): Promise<void> {
  const server = new ApolloServer({
    typeDefs,
    resolvers,
    plugins: [logRequestsPlugin],
  });
  const { url } = await startStandaloneServer(server, { listen: { port: 4444 } });
  console.info(`Server listening at ${url}...`);
}

main().catch(console.error);
