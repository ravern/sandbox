import { ApolloServerErrorCode } from '@apollo/server/errors';
import { GraphQLError } from 'graphql';

import db, { BookmarkType, CommentType, PostType, UserType } from '../db';
import { Resolvers } from '../generated/graphql';
import getObjects from './helpers/getObjects';
import paginatedResolver from './helpers/paginatedResolver';

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
  User: {
    bookmarks: paginatedResolver<UserType, BookmarkType>(user =>
      getObjects<BookmarkType>(db.bookmarks, bookmark => bookmark.userId === user.id),
    ),
    posts: paginatedResolver<UserType, PostType>(user =>
      getObjects<PostType>(db.posts, post => post.authorId === user.id),
    ),
    following: paginatedResolver<UserType, UserType>(user =>
      getObjects<UserType>(db.users, followedUser => followedUser.followers.includes(user.id)),
    ),
    followers: paginatedResolver<UserType, UserType>(user =>
      getObjects<UserType>(db.users, followingUser => user.followers.includes(followingUser.id)),
    ),
  },
  Post: {
    author: async post => {
      return db.users[post.authorId];
    },
    likes: paginatedResolver<PostType, UserType>(post =>
      getObjects<UserType>(db.users, user => post.likes.includes(user.id)),
    ),
    comments: paginatedResolver<PostType, CommentType>(post =>
      getObjects<CommentType>(db.comments, comment => comment.postId === post.id),
    ),
  },
  Comment: {
    author: async post => {
      return db.users[post.authorId];
    },
    likes: paginatedResolver<CommentType, UserType>(comment =>
      getObjects<UserType>(db.users, user => comment.likes.includes(user.id)),
    ),
  },
  Bookmark: {
    post: async bookmark => {
      return db.posts[bookmark.postId];
    },
    user: async bookmark => {
      return db.users[bookmark.userId];
    },
  },
};

export default resolvers;
