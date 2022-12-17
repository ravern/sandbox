import { ApolloClient, createHttpLink, InMemoryCache } from '@apollo/client';
import { setContext } from '@apollo/client/link/context';

const httpLink = createHttpLink({
  uri: 'http://localhost:4444',
});

const authLink = setContext((_, { headers }) => {
  const token = localStorage.getItem('token');
  if (token == null) {
    return headers;
  }
  return {
    headers: {
      ...headers,
      authorization: token ? `${token}` : "",
    }
  }
});

const cache = new InMemoryCache({
  typePolicies: {
    Query: {
      fields: {
        posts: {
          keyArgs: false,
          merge({ nodes: existingNodes = [] } = {}, { count, nextCursor, nodes: incomingNodes }) {
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
  link: authLink.concat(httpLink),
  cache,
});

export default client;