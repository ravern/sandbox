import { ApolloServerErrorCode } from '@apollo/server/errors';
import { GraphQLError } from 'graphql';

import db, { CommentType, PostType, UserType } from '../db';
import { Resolvers } from '../generated/graphql';

function paginatedResolver<P, T>(
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

function getObjects<T extends { id: string }>(
  objects: { [id: string]: T },
  predicate: (object: T) => boolean,
): { [id: string]: T } {
  const filteredObjects = {};
  Object.values(objects).forEach(object => {
    if (predicate(object)) {
      filteredObjects[object.id] = object;
    }
  });
  return filteredObjects;
}

const resolvers: Resolvers = {
  Query: {
    post: async (_parent, { id }) => {
      const post = db.posts[id];
      if (post == null) {
        throw new GraphQLError('Post not found.', {
          extensions: { code: ApolloServerErrorCode.BAD_USER_INPUT },
        });
      }
      return post;
    },
    posts: paginatedResolver<{}, PostType>(() => db.posts),
  },
  Post: {
    likes: paginatedResolver<PostType, UserType>(post =>
      getObjects<UserType>(db.users, user => post.likes.includes(user.id)),
    ),
    comments: paginatedResolver<PostType, CommentType>(post =>
      getObjects<CommentType>(db.comments, comment => comment.id === post.id),
    ),
  },
};

export default resolvers;
