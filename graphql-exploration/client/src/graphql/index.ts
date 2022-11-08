import { ApolloClient, InMemoryCache } from "@apollo/client";

const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        posts: {
          keyArgs: false,
          merge({ nodes: existingNodes = [] } = {}, { count, nextCursor, nodes: incomingNodes }) {
            console.log("merge!", existingNodes, incomingNodes);
            return {
              count,
              nextCursor,
              nodes: [...existingNodes, ...incomingNodes],
            };
          },
        },
      },
    },
  }
});

const client = new ApolloClient({
  uri: 'http://localhost:4444',
  cache,
});

export default client;