import { ApolloServerErrorCode } from '@apollo/server/errors';
import { GraphQLError } from 'graphql';

export default function paginatedResolver<P, T>(
  getObjects: (parent: P) => {
    [id: string]: T;
  },
): (
  parent: P,
  args: { limit: number; cursor?: string },
) => Promise<{ count: number; nextCursor?: string; nodes: T[] }> {
  return async (
    parent,
    { limit, cursor },
  ): Promise<{ count: number; nextCursor?: string; nodes: T[] }> => {
    const objects = getObjects(parent);
    const objectIds = Object.keys(objects);

    if (objectIds.length === 0) {
      return {
        count: 0,
        nextCursor: null,
        nodes: [],
      };
    }

    let currentCursor = cursor;
    if (currentCursor == null) {
      [currentCursor] = objectIds;
    }

    const cursorOffset = objectIds.indexOf(currentCursor);
    if (cursorOffset === -1) {
      throw new GraphQLError('Invalid cursor provided', {
        extensions: {
          code: ApolloServerErrorCode.BAD_USER_INPUT,
        },
      });
    }

    const nextCursor =
      cursorOffset + limit >= objectIds.length ? null : objectIds[cursorOffset + limit];

    return {
      count: objectIds.length,
      nextCursor,
      nodes: objectIds.slice(cursorOffset, cursorOffset + limit).map(id => objects[id]),
    };
  };
}
